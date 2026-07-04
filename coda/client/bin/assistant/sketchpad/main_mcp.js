// =============================================================================
// Building Energy Sketchpad — MCP App Edition
// =============================================================================
//
// This module handles:
//   1. MCP App bridge (communication with host via postMessage)
//   2. Zone management (add/remove rectangular zones)
//   3. 3D visualization of zones as colored volumes (Three.js)
//   4. Energy calculation via app.callServerTool()
//   5. Results display in floating overlay
//
// NO fetch('/api/...') calls — everything goes through the MCP protocol.
// NO polling — results come back directly from tool calls.
// =============================================================================

import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { DragControls } from 'three/addons/controls/DragControls.js';
import { TransformControls } from 'three/addons/controls/TransformControls.js';
import { ViewHelper } from 'three/addons/helpers/ViewHelper.js';
import polygonClipping from 'polygon-clipping';

// ── MCP App Bridge ──────────────────────────────────────────────────────────
function logToConsole(msg) {
    console.log(msg);
    const consoleEl = document.getElementById('log-console');
    if (consoleEl) {
        const time = new Date().toLocaleTimeString();
        consoleEl.innerHTML += `<div>[${time}] ${msg}</div>`;
        consoleEl.scrollTop = consoleEl.scrollHeight;
    }
}
window.logToConsole = logToConsole;

if (window.__wasmLogs) {
    window.__wasmLogs.forEach(l => logToConsole(l.msg));
    window.__wasmLogs = [];
}

// Global error logger for easier debugging
window.onerror = function (message, source, lineno, colno, error) {
    logToConsole(`[ERROR] ${message} at ${source}:${lineno}:${colno}`);
    return false;
};
window.onunhandledrejection = function (event) {
    logToConsole(`[UNHANDLED REJECTION] ${event.reason}`);
};

// Hook up clear button
function setupClearLogs() {
    const clearBtn = document.getElementById('btn-clear-logs');
    if (clearBtn) {
        clearBtn.addEventListener('click', () => {
            const consoleEl = document.getElementById('log-console');
            if (consoleEl) consoleEl.innerHTML = '';
        });
    }
}
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', setupClearLogs);
} else {
    setupClearLogs();
}

let app = null;
let AppClass = null;

try {
    logToConsole("Attempting local SDK import...");
    // Try to import from bundled/local npm package first
    const sdk = await import("@modelcontextprotocol/ext-apps");
    AppClass = sdk.App;
    logToConsole("Local SDK imported successfully.");
} catch (e) {
    logToConsole("Local SDK import failed. Attempting CDN import...");
    try {
        // Fallback to CDN for unbundled/dev/FastAPI mode
        const sdk = await import("https://cdn.jsdelivr.net/npm/@modelcontextprotocol/ext-apps/+esm");
        AppClass = sdk.App;
        logToConsole("CDN SDK imported successfully.");
    } catch (err) {
        logToConsole("CDN SDK import failed: " + err.message);
        console.warn('MCP App SDK not available:', err.message);
    }
}

if (AppClass) {
    try {
        logToConsole("Instantiating App...");
        app = new AppClass({ name: "Building Planner", version: "1.0.0" });
        logToConsole("App instantiated. Connect function type: " + typeof app.connect);
        if (typeof app.connect === 'function') {
            logToConsole("Connecting to host...");
            app.connect().then(() => {
                logToConsole("Connected! Host capabilities: " + JSON.stringify(app.getHostCapabilities()));
            }).catch(err => {
                logToConsole("Connect failed: " + err.message);
            });
        }

        // When the host sends tool result data (e.g., pre-populated settings)
        app.ontoolresult = (result) => {
            logToConsole("Received ontoolresult. Payload: " + JSON.stringify(result));
            try {
                let dataStr = result;
                if (result && typeof result === 'object') {
                    if (result.content && Array.isArray(result.content)) {
                        const textItem = result.content.find(c => c.type === 'text');
                        dataStr = textItem ? textItem.text : result;
                    } else if (result.text) {
                        dataStr = result.text;
                    }
                }
                const data = typeof dataStr === 'string' ? JSON.parse(dataStr) : dataStr;
                if (data.defaults) {
                    logToConsole("Applying default values from host...");
                    if (data.defaults.building_type) {
                        const el = document.getElementById('tabula-type');
                        if (el) {
                            el.value = data.defaults.building_type;
                        }
                    }
                    if (data.defaults.year_class) {
                        const el = document.getElementById('tabula-year');
                        if (el) {
                            if (el.tagName === 'SELECT') {
                                for (const opt of el.options) {
                                    const val = opt.value || opt.text;
                                    if (val === data.defaults.year_class || opt.text === data.defaults.year_class) {
                                        el.value = val;
                                        break;
                                    }
                                }
                            } else {
                                el.value = data.defaults.year_class;
                            }
                        }
                    }
                    if (data.defaults.scenario) {
                        const el = document.getElementById('tabula-scenario');
                        if (el) {
                            el.value = data.defaults.scenario;
                        }
                    }
                    logToConsole("Defaults applied successfully.");
                }
            } catch (e) {
                logToConsole("Error parsing/applying defaults: " + e.message);
                console.log('Could not parse tool result defaults:', e);
            }
        };
    } catch (e) {
        logToConsole("Error during App instantiation/setup: " + e.message);
        console.error('Failed to initialize MCP App:', e);
    }
}


// ── State ───────────────────────────────────────────────────────────────────
let zones = [];
let zoneCounter = 0;
let buildingRotationDeg = 0;
let wallSegments = [];
let wallWindows = { N: [], E: [], S: [], W: [] };

// ── Three.js Globals ────────────────────────────────────────────────────────
let scene, camera, renderer, controls;
let buildingGroup = new THREE.Group();
let dragControls;
let draggableObjects = [];

THREE.Object3D.DEFAULT_UP.set(0, 0, 1);
let raycaster = new THREE.Raycaster();
let mouse = new THREE.Vector2();
let translateControl;
let scaleControl;
let viewHelper;
let selectionModeActive = false;

let selectedWall = null;
let selectedWindowId = null;

window.addWindowToSelectedWall = function() {
    if (!selectedWall) return;
    const zone = zones.find(z => z.id === selectedWall.zoneId);
    if (!zone) return;
    if (!zone.windows) zone.windows = [];
    const winId = 'win_' + Date.now();
    const storyH = parseFloat(document.getElementById('story-height').value) || 2.8;
    const cx = zone.geometry.x + zone.geometry.width / 2;
    const cy = zone.geometry.y + zone.geometry.length / 2;
    const cz = storyH / 2;
    let u = 0;
    let v = selectedWall.clickPoint ? selectedWall.clickPoint.z - cz : 0;
    if (selectedWall.clickPoint) {
        if (selectedWall.wallId === 'N') u = selectedWall.clickPoint.x - cx;
        else if (selectedWall.wallId === 'S') u = cx - selectedWall.clickPoint.x;
        else if (selectedWall.wallId === 'E') u = cy - selectedWall.clickPoint.y;
        else if (selectedWall.wallId === 'W') u = selectedWall.clickPoint.y - cy;
    }
    zone.windows.push({
        id: winId,
        wall_id: selectedWall.wallId,
        u: u,
        v: v,
        width: 1.5,
        height: 1.5
    });
    rebuildAndRetainSelection(winId);
};

window.removeWindow = function(zoneId, winId) {
    const zone = zones.find(z => z.id === zoneId);
    if (zone && zone.windows) {
        zone.windows = zone.windows.filter(w => w.id !== winId);
    }
    selectedWindowId = null;
    selectedObject = null; selectedWall = null; selectedWindowId = null; updateSelectionPanel();
    translateControl.detach();
    scaleControl.detach();
    rebuildAndRetainSelection();
};

function updateSelectionPanel() {
    const panel = document.getElementById('selection-panel');
    const title = document.getElementById('selection-title');
    const content = document.getElementById('selection-content');
    if (!panel) return;

    if (selectedWindowId) {
        panel.style.display = 'block';
        title.textContent = 'Selected: Window';
        const zone = zones.find(z => z.windows && z.windows.find(w => w.id === selectedWindowId));
        const w = zone && zone.windows.find(win => win.id === selectedWindowId);
        if (!zone || !w) { panel.style.display = 'none'; return; }
        const wallLabel = { N: 'North', S: 'South', E: 'East', W: 'West' }[w.wall_id] || w.wall_id;
        content.innerHTML = `
            <div style="font-size:0.7rem; color:#64748b; margin-bottom:8px;">Wall: <span style="color:#facc15;font-weight:600;">${wallLabel}</span></div>
            <div class="form-row">
                <div><label>Width (m)</label><input type="number" id="win-width" value="${w.width.toFixed(2)}" step="0.1" min="0.1" onchange="updateWindowProperty('${zone.id}','${w.id}','width',+this.value)"></div>
                <div><label>Height (m)</label><input type="number" id="win-height" value="${w.height.toFixed(2)}" step="0.1" min="0.1" onchange="updateWindowProperty('${zone.id}','${w.id}','height',+this.value)"></div>
            </div>
            <div class="form-row">
                <div><label>Horiz. offset (m)</label><input type="number" id="win-u" value="${w.u.toFixed(2)}" step="0.05" onchange="updateWindowProperty('${zone.id}','${w.id}','u',+this.value)"></div>
                <div><label>Vert. offset (m)</label><input type="number" id="win-v" value="${w.v.toFixed(2)}" step="0.05" onchange="updateWindowProperty('${zone.id}','${w.id}','v',+this.value)"></div>
            </div>
            <button class="btn-primary" style="background:#ef4444;color:#fff;margin-top:4px;" onclick="removeWindow('${zone.id}','${selectedWindowId}')">Delete Window</button>
        `;
    } else if (selectedWall) {
        panel.style.display = 'block';
        const wallLabel = { N: 'North', S: 'South', E: 'East', W: 'West' }[selectedWall.wallId] || selectedWall.wallId;
        title.textContent = `Selected: ${wallLabel} Wall`;
        content.innerHTML = `
            <div style="font-size:0.7rem; color:#64748b; margin-bottom:8px;">Zone: ${selectedWall.zoneId}</div>
            <button class="btn-primary" onclick="addWindowToSelectedWall()">+ Add Window Here</button>
        `;
    } else if (selectedObject && selectedObject.userData.type === 'zone') {
        panel.style.display = 'block';
        title.textContent = `Selected: Zone (${selectedObject.userData.zoneId})`;
        content.innerHTML = `
            <div style="font-size:0.8rem; color:#94a3b8;">
                Use the Gizmo to move or scale the entire zone. Click on a specific wall to add windows.
            </div>
        `;
    } else {
        panel.style.display = 'none';
    }
}

window.updateWindowProperty = function(zoneId, winId, prop, val) {
    const zone = zones.find(z => z.id === zoneId);
    if (!zone) return;
    const w = zone.windows && zone.windows.find(win => win.id === winId);
    if (!w) return;
    w[prop] = val;
    selectedWindowId = winId;
    rebuildAndRetainSelection(winId);
};

let selectedObject = null; selectedWall = null; selectedWindowId = null; updateSelectionPanel();

const ZONE_COLORS = [
    0x38bdf8, 0x818cf8, 0xc084fc, 0xf472b6,
    0xfb923c, 0xfbbf24, 0x34d399, 0x2dd4bf,
    0x60a5fa, 0xa78bfa, 0xe879f9, 0xf87171,
];


// ─────────────────────────────────────────────────────────────────────────────
// Toast Notifications
// ─────────────────────────────────────────────────────────────────────────────

function showToast(message, isError = false, durationMs = 4000) {
    const toast = document.getElementById('toast');
    toast.textContent = message;
    toast.className = isError ? 'toast error visible' : 'toast visible';
    setTimeout(() => { toast.className = 'toast'; }, durationMs);
}


// ─────────────────────────────────────────────────────────────────────────────
// Three.js Setup
// ─────────────────────────────────────────────────────────────────────────────

function initThree() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x0a0f1e);

    const container = document.getElementById('canvas-container');
    const rect = container.getBoundingClientRect();

    const aspect = rect.width / rect.height;
    const frustumSize = 30;
    camera = new THREE.OrthographicCamera((frustumSize * aspect) / -2, (frustumSize * aspect) / 2, frustumSize / 2, frustumSize / -2, -1000, 1000);
    camera.position.set(20, 20, 20); // Isometric perspective
    camera.up.set(0, 0, 1);

    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(rect.width, rect.height);
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.shadowMap.enabled = true;
    renderer.autoClear = false;
    container.appendChild(renderer.domElement);

    controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.08;
    controls.target.set(0, 0, 0);
    // Prevent Gimbal lock when looking straight down or up
    controls.minPolarAngle = 0.001;
    controls.maxPolarAngle = Math.PI - 0.001;

    // Lights
    scene.add(new THREE.AmbientLight(0xffffff, 0.5));
    const dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
    dirLight.position.set(15, 25, 15);
    dirLight.castShadow = true;
    scene.add(dirLight);
    const fillLight = new THREE.DirectionalLight(0x818cf8, 0.3);
    fillLight.position.set(-10, 5, -10);
    scene.add(fillLight);

    // Grid
    const grid = new THREE.GridHelper(30, 30, 0x1e293b, 0x0f172a);
    grid.rotation.x = Math.PI / 2;
    grid.position.z = -0.01;
    scene.add(grid);

    // North Arrow
    const dir = new THREE.Vector3(0, 1, 0);
    const origin = new THREE.Vector3(-12, -12, 0);
    window.northArrow = new THREE.ArrowHelper(dir, origin, 3, 0xef4444, 1, 0.8);
    scene.add(window.northArrow);

    scene.add(buildingGroup);

    // Resize
    window.addEventListener('resize', () => {
        const r = container.getBoundingClientRect();
        const aspect = r.width / r.height;
        const frustumSize = 30;
        camera.left = (frustumSize * aspect) / -2;
        camera.right = (frustumSize * aspect) / 2;
        camera.top = frustumSize / 2;
        camera.bottom = frustumSize / -2;
        camera.updateProjectionMatrix();
        renderer.setSize(r.width, r.height);
    });

    // Drag Controls
    dragControls = new DragControls(draggableObjects, camera, renderer.domElement);
    dragControls.addEventListener('dragstart', (event) => {
        controls.enabled = false;
        event.object.material.opacity = 0.5;
    });
    dragControls.addEventListener('drag', (event) => {
        const zPos = event.object.userData.storyIndex * (parseFloat(document.getElementById('story-height').value) || 2.8) + (parseFloat(document.getElementById('story-height').value) || 2.8) / 2;
        event.object.position.z = zPos;
    });
    dragControls.addEventListener('dragend', (event) => {
        controls.enabled = true;
        event.object.material.opacity = 0.25;
        const zoneId = event.object.userData.zoneId;
        const zone = zones.find(z => z.id === zoneId);
        if (zone) {
            zone.geometry.x = Math.round((event.object.position.x - zone.geometry.width / 2) * 2) / 2;
            zone.geometry.y = Math.round((event.object.position.y - zone.geometry.length / 2) * 2) / 2;
            renderZoneList();
            render3DZones();
            updateStats();
            if (typeof dispatchState === 'function') dispatchState();
        }
    });

    // Setup TransformControls (Gumball)
    translateControl = new TransformControls(camera, renderer.domElement);
    scaleControl = new TransformControls(camera, renderer.domElement);
    scaleControl.setMode('scale');
    translateControl.setSpace('local');
    scaleControl.setSpace('local');
    
    // Hide Z axis handles since building zones are constrained to XY plane manipulation
    translateControl.showZ = false;
    scaleControl.showZ = false;

    translateControl.addEventListener('dragging-changed', function (event) {
        controls.enabled = !event.value;
    });
    scaleControl.addEventListener('dragging-changed', function (event) {
        controls.enabled = !event.value;
    });
    translateControl.addEventListener('change', function () {
        if (selectedObject && translateControl.object) {
            const storyH = parseFloat(document.getElementById('story-height').value) || 2.8;
            const zPos = selectedObject.userData.storyIndex * storyH + storyH / 2;
            selectedObject.position.z = zPos; // lock Z axis
        }
    });
    
    function rebuildAndRetainSelection(targetId) {
        renderZoneList();
        render3DZones();
        updateStats();
        if (typeof dispatchState === 'function') dispatchState();
        const newMesh = draggableObjects.find(obj =>
            (obj.userData.type === 'window' && obj.userData.windowId === targetId) ||
            (obj.userData.type === 'zone' && obj.userData.zoneId === targetId)
        );
        if (newMesh) {
            selectedObject = newMesh;
            if (window.activeTransformMode === 'scale') {
                scaleControl.attach(selectedObject);
                translateControl.detach();
            } else {
                translateControl.attach(selectedObject);
                scaleControl.detach();
            }
        } else {
            translateControl.detach();
            scaleControl.detach();
            selectedObject = null; selectedWall = null; selectedWindowId = null; updateSelectionPanel();
        }
    }

    translateControl.addEventListener('mouseUp', function () {
        if (selectedObject) {
            if (selectedObject.userData.type === 'zone') {
                const zoneId = selectedObject.userData.zoneId;
                const zone = zones.find(z => z.id === zoneId);
                if (zone) {
                    zone.geometry.x = Math.round((selectedObject.position.x - zone.geometry.width / 2) * 2) / 2;
                    zone.geometry.y = Math.round((selectedObject.position.y - zone.geometry.length / 2) * 2) / 2;
                    rebuildAndRetainSelection(zoneId);
                }
            } else if (selectedObject.userData.type === 'window') {
                const zone = zones.find(z => z.id === selectedObject.userData.zoneId);
                if (zone) {
                    const w = zone.windows.find(win => win.id === selectedObject.userData.windowId);
                    if (w) {
                        const storyH = parseFloat(document.getElementById('story-height').value) || 2.8;
                        const cx = zone.geometry.x + zone.geometry.width/2;
                        const cy = zone.geometry.y + zone.geometry.length/2;
                        const cz = storyH / 2;

                        if (w.wall_id === 'N') {
                            w.u = selectedObject.position.x - cx;
                            w.v = selectedObject.position.z - cz;
                        } else if (w.wall_id === 'S') {
                            w.u = cx - selectedObject.position.x;
                            w.v = selectedObject.position.z - cz;
                        } else if (w.wall_id === 'E') {
                            w.u = cy - selectedObject.position.y;
                            w.v = selectedObject.position.z - cz;
                        } else if (w.wall_id === 'W') {
                            w.u = selectedObject.position.y - cy;
                            w.v = selectedObject.position.z - cz;
                        } else if (w.wall_id === 'H') {
                            w.u = selectedObject.position.x - cx;
                            w.v = selectedObject.position.y - cy;
                        }
                        rebuildAndRetainSelection(w.id);
                    }
                }
            }
        }
    });

    scaleControl.addEventListener('mouseUp', function () {
        if (selectedObject) {
            if (selectedObject.userData.type === 'zone') {
                const zoneId = selectedObject.userData.zoneId;
                const zone = zones.find(z => z.id === zoneId);
                if (zone) {
                    const scaleX = selectedObject.scale.x;
                    const scaleY = selectedObject.scale.y;
                    zone.geometry.width = Math.max(0.5, Math.round(zone.geometry.width * scaleX * 10) / 10);
                    zone.geometry.length = Math.max(0.5, Math.round(zone.geometry.length * scaleY * 10) / 10);
                    rebuildAndRetainSelection(zoneId);
                }
            } else if (selectedObject.userData.type === 'window') {
                const zone = zones.find(z => z.id === selectedObject.userData.zoneId);
                if (zone) {
                    const w = zone.windows.find(win => win.id === selectedObject.userData.windowId);
                    if (w) {
                        w.width = Math.max(0.1, Math.round(w.width * selectedObject.scale.x * 10) / 10);
                        w.height = Math.max(0.1, Math.round(w.height * selectedObject.scale.y * 10) / 10);
                        rebuildAndRetainSelection(w.id);
                    }
                }
            }
        }
    });

    scene.add(translateControl);
    scene.add(scaleControl);

    // Setup ViewHelper
    viewHelper = new ViewHelper(camera, renderer.domElement);
    viewHelper.center = controls.target;

    // Re-implement ViewHelper.handleClick from scratch.
    // We cannot use the native one because `targetPosition` is private inside the class,
    // and the native `update` method forces a Y-Up orientation resulting in gimbal lock for our Z-Up scene.
    viewHelper.handleClick = function(event) {
        if (this.animating) return false;

        const dim = 128;
        const rect = renderer.domElement.getBoundingClientRect();
        const offsetX = rect.left + (rect.width - dim);
        const offsetY = rect.top + (rect.height - dim);

        const mouse = new THREE.Vector2();
        mouse.x = ((event.clientX - offsetX) / dim) * 2 - 1;
        mouse.y = -((event.clientY - offsetY) / dim) * 2 + 1;

        // If the click is strictly outside the ViewHelper's 128x128 box, ignore it
        if (event.clientX < offsetX || event.clientY < offsetY) return false;

        const orthoCamera = new THREE.OrthographicCamera(-2, 2, 2, -2, 0, 4);
        orthoCamera.position.set(0, 0, 2);

        const raycaster = new THREE.Raycaster();
        raycaster.setFromCamera(mouse, orthoCamera);

        const intersects = raycaster.intersectObjects(this.children, true);
        const hit = intersects.find(i => i.object.userData && i.object.userData.type);

        if (hit) {
            let type = hit.object.userData.type;
            let targetDir = new THREE.Vector3();
            if (type === 'posX') targetDir.set(1, 0, 0);
            else if (type === 'posY') targetDir.set(0, 1, 0);
            else if (type === 'posZ') targetDir.set(0, 0, 1);
            else if (type === 'negX') targetDir.set(-1, 0, 0);
            else if (type === 'negY') targetDir.set(0, -1, 0);
            else if (type === 'negZ') targetDir.set(0, 0, -1);
            else return false;

            // Add tiny offsets to prevent OrbitControls from hitting perfect 0 or 180 polar angles (Gimbal Lock)
            if (Math.abs(targetDir.z) > 0.99) {
                targetDir.x += 0.001;
                targetDir.y += 0.001;
            } else {
                targetDir.z += 0.001;
            }
            targetDir.normalize();

            const dist = camera.position.distanceTo(controls.target);
            camera.position.copy(controls.target).add(targetDir.multiplyScalar(dist));
            
            camera.up.set(0, 0, 1);
            camera.lookAt(controls.target);
            camera.updateProjectionMatrix();
            controls.update();
            
            return true;
        }
        return false;
    };

    container.addEventListener('pointerup', (event) => {
        viewHelper.handleClick(event);
    });

    container.addEventListener('pointerdown', (event) => {
        if (!selectionModeActive) return;

        const rect = container.getBoundingClientRect();
        mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
        mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

        raycaster.setFromCamera(mouse, camera);

        const intersects = raycaster.intersectObjects(draggableObjects, false);
        if (intersects.length > 0) {
            const object = intersects[0].object;
            const userData = object.userData;
            
            if (userData.type === 'window') {
                selectedWindowId = userData.windowId;
                selectedWall = null;
                selectedObject = object;
                rebuildAndRetainSelection(selectedWindowId);
            } else if (userData.type === 'zone') {
                const normalMatrix = new THREE.Matrix3().getNormalMatrix(object.matrixWorld);
                const worldNormal = intersects[0].face.normal.clone().applyMatrix3(normalMatrix).normalize();
                const ax = Math.abs(worldNormal.x);
                const ay = Math.abs(worldNormal.y);
                const az = Math.abs(worldNormal.z);
                let wallId = null;
                if (az < 0.5) {
                    if (ay >= ax) wallId = worldNormal.y > 0 ? 'N' : 'S';
                    else wallId = worldNormal.x > 0 ? 'E' : 'W';
                } else if (worldNormal.z > 0.5) {
                    wallId = 'H';
                }

                if (wallId) {
                    selectedWall = { zoneId: userData.zoneId, wallId, clickPoint: intersects[0].point.clone() };
                    selectedWindowId = null;
                    selectedObject = null;
                    translateControl.detach();
                    scaleControl.detach();
                    updateSelectionPanel();
                    render3DZones();
                } else {
                    selectedWall = null;
                    selectedWindowId = null;
                    selectedObject = object;
                    rebuildAndRetainSelection(userData.zoneId);
                }
            }
        } else {
            selectedObject = null;
            selectedWall = null;
            selectedWindowId = null;
            translateControl.detach();
            scaleControl.detach();
            updateSelectionPanel();
            render3DZones();
        }
    });

    animate();
}

let clock = new THREE.Clock();

function animate() {
    requestAnimationFrame(animate);
    
    const delta = clock.getDelta();
    if (viewHelper && viewHelper.animating === true) {
        viewHelper.update(delta);
    } else {
        controls.update();
    }
    
    renderer.clear();
    renderer.render(scene, camera);
    if (viewHelper) viewHelper.render(renderer);
}


// ─────────────────────────────────────────────────────────────────────────────
// Zone Management
// ─────────────────────────────────────────────────────────────────────────────

function addZone(type, x, y, width, length) {
    zoneCounter++;
    const zone = { id: `zone_${zoneCounter}`, type, geometry: { x, y, width, length }, windows: [] };
    zones.push(zone);
    renderZoneList();
    render3DZones();
    updateStats();
    return zone;
}

function removeZone(id) {
    zones = zones.filter(z => z.id !== id);
    renderZoneList();
    render3DZones();
    updateStats();
}

function getGeometryPayload() {
    return { version: "1.0", units: "meters", building_zones: zones };
}


// ─────────────────────────────────────────────────────────────────────────────
// 3D Rendering
// ─────────────────────────────────────────────────────────────────────────────

function render3DZones() {
    draggableObjects.length = 0;
    while (buildingGroup.children.length > 0) {
        const child = buildingGroup.children[0];
        child.traverse(obj => {
            if (obj.geometry) obj.geometry.dispose();
            if (obj.material) {
                if (Array.isArray(obj.material)) obj.material.forEach(m => m.dispose());
                else obj.material.dispose();
            }
        });
        buildingGroup.remove(child);
    }
    
    // Detach gizmo to avoid dangling reference — preserve selectedWall so highlight redraws
    if (translateControl && translateControl.object) {
        translateControl.detach();
        scaleControl.detach();
        selectedObject = null;
        selectedWindowId = null;
    }

    const storyHeight = parseFloat(document.getElementById('story-height').value) || 2.8;
    const numStories = parseInt(document.getElementById('num-stories').value) || 1;

    zones.forEach((zone, idx) => {
        const { x, y, width, length } = zone.geometry;
        const color = ZONE_COLORS[idx % ZONE_COLORS.length];

        for (let s = 0; s < numStories; s++) {
            const boxGeom = new THREE.BoxGeometry(width, length, storyHeight);
            const boxMat = new THREE.MeshPhongMaterial({
                color, transparent: true, opacity: 0.25, side: THREE.DoubleSide,
            });
            const boxMesh = new THREE.Mesh(boxGeom, boxMat);
            boxMesh.position.set(x + width / 2, y + length / 2, s * storyHeight + storyHeight / 2);
            boxMesh.userData = { type: 'zone', zoneId: zone.id, storyIndex: s };
            draggableObjects.push(boxMesh);
            buildingGroup.add(boxMesh);

            const edgesGeom = new THREE.EdgesGeometry(boxGeom);
            const edgesMat = new THREE.LineBasicMaterial({ color, linewidth: 2 });
            const edges = new THREE.LineSegments(edgesGeom, edgesMat);
            edges.position.copy(boxMesh.position);
            buildingGroup.add(edges);
        }

        // Floor plane
        const floorGeom = new THREE.PlaneGeometry(width, length);
        const floorMat = new THREE.MeshPhongMaterial({ color, transparent: true, opacity: 0.35, side: THREE.DoubleSide });
        const floor = new THREE.Mesh(floorGeom, floorMat);
        // Z-up: PlaneGeometry defaults to XY plane, so no rotation needed!
        floor.position.set(x + width / 2, y + length / 2, 0.01);
        buildingGroup.add(floor);

        // Label sprite
        const canvas = document.createElement('canvas');
        canvas.width = 256; canvas.height = 64;
        const ctx = canvas.getContext('2d');
        ctx.fillStyle = 'rgba(0,0,0,0.6)';
        ctx.fillRect(0, 0, 256, 64);
        ctx.fillStyle = '#ffffff';
        ctx.font = 'bold 20px Inter, sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(zone.type, 128, 25);
        ctx.font = '16px Inter, sans-serif';
        ctx.fillStyle = '#94a3b8';
        ctx.fillText(`${width}m × ${length}m`, 128, 50);
        const texture = new THREE.CanvasTexture(canvas);
        const spriteMat = new THREE.SpriteMaterial({ map: texture, transparent: true });
        const sprite = new THREE.Sprite(spriteMat);
        sprite.scale.set(3, 0.75, 1);
        sprite.position.set(x + width / 2, y + length / 2, numStories * storyHeight + 0.8);
        buildingGroup.add(sprite);

        if (!zone.windows) zone.windows = [];
        zone.windows.forEach(w => {
            const wGeom = new THREE.PlaneGeometry(w.width, w.height);
            const isSelected = (selectedWindowId === w.id);
            const wColor = isSelected ? 0xef4444 : 0x38bdf8;
            const wMat = new THREE.MeshPhongMaterial({ color: wColor, transparent: true, opacity: 0.9, side: THREE.DoubleSide });
            const wMesh = new THREE.Mesh(wGeom, wMat);
            wMesh.userData = { type: 'window', windowId: w.id, zoneId: zone.id, wallId: w.wall_id };
            
            const cx = x + width/2;
            const cy = y + length/2;
            const cz = storyHeight / 2;
            
            let wx, wy, wz;
            let rotX = 0, rotY = 0;
            const offset = 0.05;

            if (w.wall_id === 'N') {
                wx = cx + w.u;
                wy = y + length + offset;
                wz = cz + w.v;
                rotX = -Math.PI/2;
            } else if (w.wall_id === 'S') {
                wx = cx - w.u;
                wy = y - offset;
                wz = cz + w.v;
                rotX = Math.PI/2;
            } else if (w.wall_id === 'E') {
                wx = x + width + offset;
                wy = cy - w.u;
                wz = cz + w.v;
                rotY = Math.PI/2;
            } else if (w.wall_id === 'W') {
                wx = x - offset;
                wy = cy + w.u;
                wz = cz + w.v;
                rotY = -Math.PI/2;
            } else if (w.wall_id === 'H') {
                wx = cx + w.u;
                wy = cy + w.v;
                wz = (numStories * storyHeight) + offset;
                rotX = 0;
                rotY = 0;
            }
            
            wMesh.position.set(wx, wy, wz);
            wMesh.rotation.set(rotX, rotY, 0);
            
            draggableObjects.push(wMesh);
            buildingGroup.add(wMesh);
        });

        if (selectedWall && selectedWall.zoneId === zone.id) {
            const wallId = selectedWall.wallId;
            const isN_S = (wallId === 'N' || wallId === 'S');
            const isH = (wallId === 'H');
            const fullHeight = storyHeight * numStories;
            
            let wallGeom;
            if (isH) {
                wallGeom = new THREE.PlaneGeometry(width, length);
            } else {
                const wWidth = isN_S ? width : length;
                wallGeom = new THREE.PlaneGeometry(wWidth, fullHeight);
            }
            const wallMat = new THREE.MeshBasicMaterial({
                color: 0xfacc15,
                transparent: true,
                opacity: 0.35,
                side: THREE.DoubleSide,
                depthWrite: false
            });
            const wallMesh = new THREE.Mesh(wallGeom, wallMat);
            
            const cx = x + width/2;
            const cy = y + length/2;
            const cz = fullHeight / 2;
            
            let wx, wy, wz;
            let rotX = 0, rotY = 0;
            const offset = 0.02;
            
            if (wallId === 'N') {
                wx = cx;
                wy = y + length + offset;
                wz = cz;
                rotX = -Math.PI/2;
            } else if (wallId === 'S') {
                wx = cx;
                wy = y - offset;
                wz = cz;
                rotX = Math.PI/2;
            } else if (wallId === 'E') {
                wx = x + width + offset;
                wy = cy;
                wz = cz;
                rotY = Math.PI/2;
            } else if (wallId === 'W') {
                wx = x - offset;
                wy = cy;
                wz = cz;
                rotY = -Math.PI/2;
            } else if (wallId === 'H') {
                wx = cx;
                wy = cy;
                wz = fullHeight + offset;
                rotX = 0;
                rotY = 0;
            }
            
            wallMesh.position.set(wx, wy, wz);
            wallMesh.rotation.set(rotX, rotY, 0);
            
            const edgesGeom = new THREE.EdgesGeometry(wallGeom);
            const edgesMat = new THREE.LineBasicMaterial({ color: 0xeab308, linewidth: 3 });
            const edges = new THREE.LineSegments(edgesGeom, edgesMat);
            wallMesh.add(edges);
            
            buildingGroup.add(wallMesh);
        }

    });

    if (zones.length > 0) {
        const bbox = new THREE.Box3().setFromObject(buildingGroup);
        const center = bbox.getCenter(new THREE.Vector3());
        controls.target.copy(center);
    }
}


// ─────────────────────────────────────────────────────────────────────────────
// Zone List UI
// ─────────────────────────────────────────────────────────────────────────────

function renderZoneList() {
    const container = document.getElementById('zone-list');
    container.innerHTML = '';
    zones.forEach((zone, idx) => {
        const color = ZONE_COLORS[idx % ZONE_COLORS.length];
        const hexColor = '#' + color.toString(16).padStart(6, '0');
        const card = document.createElement('div');
        card.className = 'zone-card';
        card.style.borderLeftColor = hexColor;
        card.style.borderLeftWidth = '3px';
        card.innerHTML = `
            <div class="zone-card-header">
                <span>${zone.id}</span>
                <span class="zone-card-type">${zone.type}</span>
                <button class="btn-delete" data-id="${zone.id}">✕</button>
            </div>
            <div class="dims">
                <span>${zone.geometry.width}m</span> × <span>${zone.geometry.length}m</span>
                at (${zone.geometry.x}, ${zone.geometry.y})
                = <span>${(zone.geometry.width * zone.geometry.length).toFixed(1)} m²</span>
            </div>
        `;
        container.appendChild(card);
    });
    document.querySelectorAll('.btn-delete').forEach(btn => {
        btn.addEventListener('click', (e) => removeZone(e.target.dataset.id));
    });
}


// ─────────────────────────────────────────────────────────────────────────────
// Status Bar
// ─────────────────────────────────────────────────────────────────────────────

function updateStats() {
    document.getElementById('stat-zones').textContent = zones.length;
    if (zones.length === 0) {
        document.getElementById('stat-area').textContent = '–';
        if (document.getElementById('stat-perimeter')) document.getElementById('stat-perimeter').textContent = '–';
        return;
    }
    const totalArea = zones.reduce((sum, z) => sum + z.geometry.width * z.geometry.length, 0);
    document.getElementById('stat-area').textContent = totalArea.toFixed(1) + ' m²';
    const totalPerimeter = zones.reduce((sum, z) => sum + 2 * (z.geometry.width + z.geometry.length), 0);
    if (document.getElementById('stat-perimeter')) {
        document.getElementById('stat-perimeter').textContent = totalPerimeter.toFixed(1) + ' m';
    }
}


// ─────────────────────────────────────────────────────────────────────────────
// Rust WASM State Integration
// ─────────────────────────────────────────────────────────────────────────────

function getRustStatePayload() {
    const story_height = parseFloat(document.getElementById('story-height').value) || 2.8;
    const num_stories = parseInt(document.getElementById('num-stories').value) || 1;
    const params = {
        building_type: document.getElementById('tabula-type').value,
        year_class: document.getElementById('tabula-year').value,
        scenario: document.getElementById('tabula-scenario').value,
        story_height,
        num_stories,
        window_to_wall_ratio: 0.15,
        building_rotation_deg: buildingRotationDeg,
        heating_system: document.getElementById('heating-system').value,
        thermal_bridge_category: document.getElementById('thermal-bridge').value,
        ground_contact_type: document.getElementById('ground-contact').value,
        shutter_control: document.getElementById('shutter-control').value,
        climate_region: document.getElementById('climate-region').value,
        usage_profile: document.getElementById('usage-profile').value,
        automation_class: document.getElementById('automation-class').value,
        
        // Ventilation
        air_tightness: document.getElementById('air-tightness') ? document.getElementById('air-tightness').value : 'CategoryII',
        has_atd: document.getElementById('has-atd') ? document.getElementById('has-atd').checked : false,
        mech_supply: document.getElementById('mech-supply') ? parseFloat(document.getElementById('mech-supply').value) || 0 : 0,
        mech_exhaust: document.getElementById('mech-exhaust') ? parseFloat(document.getElementById('mech-exhaust').value) || 0 : 0,
        heat_recovery: document.getElementById('heat-recovery') ? (parseFloat(document.getElementById('heat-recovery').value) || 0) / 100.0 : 0,
        mech_hours: document.getElementById('mech-hours') ? parseFloat(document.getElementById('mech-hours').value) || 0 : 0,
        
        // Internal Heat Gains
        lighting_exhaust: document.getElementById('lighting-exhaust') ? document.getElementById('lighting-exhaust').value : 'Standard',
        material_transport: document.getElementById('material-transport') ? document.getElementById('material-transport').value : 'None',
        custom_occupants: document.getElementById('custom-occupants') ? parseFloat(document.getElementById('custom-occupants').value) || 0 : 0,
        custom_equipment: document.getElementById('custom-equipment') ? parseFloat(document.getElementById('custom-equipment').value) || 0 : 0,
    };

    const wallMat = document.getElementById('custom-wall-mat') ? document.getElementById('custom-wall-mat').value : 'none';
    const wallThick = document.getElementById('custom-wall-thick') ? parseFloat(document.getElementById('custom-wall-thick').value) || 0 : 0;
    if (wallMat !== 'none' && wallThick > 0) {
        params.custom_wall_insulation = {
            thickness_m: wallThick,
            lambda: parseFloat(wallMat)
        };
    } else {
        params.custom_wall_insulation = null;
    }

    const roofMat = document.getElementById('custom-roof-mat') ? document.getElementById('custom-roof-mat').value : 'none';
    const roofThick = document.getElementById('custom-roof-thick') ? parseFloat(document.getElementById('custom-roof-thick').value) || 0 : 0;
    if (roofMat !== 'none' && roofThick > 0) {
        params.custom_roof_insulation = {
            thickness_m: roofThick,
            lambda: parseFloat(roofMat)
        };
    } else {
        params.custom_roof_insulation = null;
    }

    const floorMat = document.getElementById('custom-floor-mat') ? document.getElementById('custom-floor-mat').value : 'none';
    const floorThick = document.getElementById('custom-floor-thick') ? parseFloat(document.getElementById('custom-floor-thick').value) || 0 : 0;
    if (floorMat !== 'none' && floorThick > 0) {
        params.custom_floor_insulation = {
            thickness_m: floorThick,
            lambda: parseFloat(floorMat)
        };
    } else {
        params.custom_floor_insulation = null;
    }
    
    let polys = zones.map(z => [[
        [z.geometry.x, z.geometry.y],
        [z.geometry.x + z.geometry.width, z.geometry.y],
        [z.geometry.x + z.geometry.width, z.geometry.y + z.geometry.length],
        [z.geometry.x, z.geometry.y + z.geometry.length],
        [z.geometry.x, z.geometry.y]
    ]]);
    
    let unionPolys = [];
    if (polys.length > 0) {
        unionPolys = polygonClipping.union(...polys);
    }
    
    const rot_rad = -buildingRotationDeg * Math.PI / 180;
    
    let floor_area_2d = 0;
    let exterior_perimeter = 0;
    
    let envelope_data = {
        N: { gross_wall_area: 0, window_area: 0 },
        E: { gross_wall_area: 0, window_area: 0 },
        S: { gross_wall_area: 0, window_area: 0 },
        W: { gross_wall_area: 0, window_area: 0 },
        H: { gross_wall_area: 0, window_area: 0 }
    };
    
    unionPolys.forEach((multi, idx) => {
        multi.forEach(ring => {
            let ring_area = 0;
            for (let i = 0; i < ring.length - 1; i++) {
                ring_area += ring[i][0] * ring[i+1][1] - ring[i+1][0] * ring[i][1];
            }
            floor_area_2d += Math.abs(ring_area / 2);
            
            for (let i = 0; i < ring.length - 1; i++) {
                let p1 = ring[i];
                let p2 = ring[i+1];
                let dx = p2[0] - p1[0];
                let dy = p2[1] - p1[1];
                let length = Math.sqrt(dx*dx + dy*dy);
                exterior_perimeter += length;
                
                let gross_wall_area = length * story_height * num_stories;
                
                let dx_rot = dx * Math.cos(rot_rad) - dy * Math.sin(rot_rad);
                let dy_rot = dx * Math.sin(rot_rad) + dy * Math.cos(rot_rad);
                let normal_x = dy_rot;
                let normal_y = -dx_rot;
                let angle_deg = Math.atan2(normal_x, normal_y) * 180 / Math.PI;
                if (angle_deg < 0) angle_deg += 360;
                
                let cardinal = "N";
                if (angle_deg >= 315 || angle_deg < 45) cardinal = "N";
                else if (angle_deg >= 45 && angle_deg < 135) cardinal = "E";
                else if (angle_deg >= 135 && angle_deg < 225) cardinal = "S";
                else cardinal = "W";
                
                envelope_data[cardinal].gross_wall_area += gross_wall_area;
                
                let mx = (p1[0] + p2[0]) / 2;
                let my = (p1[1] + p2[1]) / 2;
                
                zones.forEach(z => {
                    const eps = 0.001;
                    if (mx >= z.geometry.x - eps && mx <= z.geometry.x + z.geometry.width + eps &&
                        my >= z.geometry.y - eps && my <= z.geometry.y + z.geometry.length + eps) {
                        
                        let unrot_nx = dy;
                        let unrot_ny = -dx;
                        let unrot_ang = Math.atan2(unrot_nx, unrot_ny) * 180 / Math.PI;
                        if (unrot_ang < 0) unrot_ang += 360;
                        let orig_card = "N";
                        if (unrot_ang >= 315 || unrot_ang < 45) orig_card = "N";
                        else if (unrot_ang >= 45 && unrot_ang < 135) orig_card = "E";
                        else if (unrot_ang >= 135 && unrot_ang < 225) orig_card = "S";
                        else orig_card = "W";
                        
                        (z.windows || []).forEach(w => {
                            if (w.wall_id === orig_card) {
                                if (!z._windowsHandled) z._windowsHandled = new Set();
                                if (!z._windowsHandled.has(w.id)) {
                                    envelope_data[cardinal].window_area += (w.width * w.height) * num_stories;
                                    z._windowsHandled.add(w.id);
                                }
                            }
                        });
                    }
                });
            }
        });
    });
    
    // Add roof windows (skylights) which don't map to a vertical wall
    envelope_data.H.gross_wall_area = floor_area_2d; // Total roof area is the floor area
    zones.forEach(z => {
        (z.windows || []).forEach(w => {
            if (w.wall_id === 'H') {
                if (!z._windowsHandled) z._windowsHandled = new Set();
                if (!z._windowsHandled.has(w.id)) {
                    envelope_data.H.window_area += (w.width * w.height);
                    z._windowsHandled.add(w.id);
                }
            }
        });
    });
    
    let total_floor_area = floor_area_2d * num_stories;
    let total_roof_area = floor_area_2d;
    let total_ground_area = floor_area_2d;
    let total_conditioned_volume = total_floor_area * story_height;
    
    let buildingGeometry = {
        total_conditioned_volume,
        total_floor_area,
        total_roof_area,
        total_ground_area,
        exterior_perimeter,
        envelope_data
    };
    
    zones.forEach(z => { if (z._windowsHandled) delete z._windowsHandled; });
    
    return { geometry: buildingGeometry, params, ui_state: { raw_zones: zones } };
}

function syncUIFromState(state) {
    if (state.ui_state && state.ui_state.raw_zones) {
        zones = state.ui_state.raw_zones;
    } else {
        zones = state.zones.map(z => ({
            id: z.id,
            type: z.room_type,
            geometry: { width: 5, length: 5, x: 0, y: 0 },
            windows: []
        }));
    }
    zoneCounter = zones.length;
    
    document.getElementById('tabula-type').value = state.params.building_type;
    document.getElementById('tabula-year').value = state.params.year_class;
    document.getElementById('tabula-scenario').value = state.params.scenario;
    document.getElementById('story-height').value = state.params.story_height;
    document.getElementById('num-stories').value = state.params.num_stories;
    buildingRotationDeg = state.params.building_rotation_deg;
    document.getElementById('heating-system').value = state.params.heating_system;
    document.getElementById('thermal-bridge').value = state.params.thermal_bridge_category || "Standard Default";
    document.getElementById('ground-contact').value = state.params.ground_contact_type || "Unheated Basement";
    document.getElementById('shutter-control').value = state.params.shutter_control || "Manual";
    document.getElementById('climate-region').value = state.params.climate_region || "Potsdam";
    document.getElementById('usage-profile').value = state.params.usage_profile || "Residential";
    document.getElementById('automation-class').value = state.params.automation_class || "C";
    
    if (document.getElementById('lighting-exhaust')) document.getElementById('lighting-exhaust').value = state.params.lighting_exhaust || "Standard";
    if (document.getElementById('material-transport')) document.getElementById('material-transport').value = state.params.material_transport || "None";
    if (document.getElementById('custom-occupants')) document.getElementById('custom-occupants').value = state.params.custom_occupants || 0;
    if (document.getElementById('custom-equipment')) document.getElementById('custom-equipment').value = state.params.custom_equipment || 0;

    const cwCheckbox = document.getElementById('custom-wall-override');
    const cwLambda = document.getElementById('custom-wall-lambda');
    const cwThickness = document.getElementById('custom-wall-thickness');
    
    if (state.params.custom_wall_insulation) {
        cwCheckbox.checked = true;
        cwLambda.value = state.params.custom_wall_insulation.lambda;
        cwThickness.value = state.params.custom_wall_insulation.thickness_m;
        cwLambda.disabled = false;
        cwThickness.disabled = false;
    } else {
        cwCheckbox.checked = false;
        cwLambda.disabled = true;
        cwThickness.disabled = true;
    }
    
    const thicknessDisplay = document.getElementById('thickness-display');
    if (thicknessDisplay && cwThickness) {
        thicknessDisplay.innerText = Math.round(parseFloat(cwThickness.value || 0) * 100) + ' cm';
    }
    
    renderZoneList();
    render3DZones();
    updateStats();
}

window.visNetwork = null;
window.currentSuggestions = [];

function initGraph(graphData) {
    const container = document.getElementById('graph-container');
    if (!container || !window.vis) return;
    const data = {
        nodes: new vis.DataSet(graphData.nodes),
        edges: new vis.DataSet(graphData.edges)
    };
    const options = {
        nodes: { shape: 'box', margin: 10, font: { color: '#ffffff' } },
        edges: { color: '#38bdf8', arrows: 'to' },
        groups: {
            Building: { color: { background: '#ef4444', border: '#b91c1c' } },
            Space: { color: { background: '#38bdf8', border: '#0284c7' } },
            Wall: { color: { background: '#fbbf24', border: '#b45309' } },
            Roof: { color: { background: '#f59e0b', border: '#b45309' } },
            Slab: { color: { background: '#d97706', border: '#b45309' } },
            Window: { color: { background: '#818cf8', border: '#4f46e5' } },
            Calculation: { shape: 'diamond', font: { size: 16, bold: true }, color: { background: '#22c55e', border: '#166534' } }
        },
        physics: { stabilization: true }
    };
    window.visNetwork = new vis.Network(container, data, options);

    // Setup click handler for physics documentation
    window.visNetwork.on("click", function (params) {
        const infoPanel = document.getElementById('info-panel');
        if (params.nodes.length > 0) {
            const nodeId = params.nodes[0];
            const nodesDataset = window.visNetwork.body.data.nodes;
            const node = nodesDataset.get(nodeId);
            
            if (node && node.doc) {
                const infoTitle = document.getElementById('info-title');
                const infoContent = document.getElementById('info-content');
                
                // Extract property name cleanly
                infoTitle.innerText = node.label.split(':')[0];
                
                // Simple markdown-to-HTML parser for the doc
                let htmlDoc = node.doc.replace(/\*\*(.*?)\*\*/g, '<b style="color:white;">$1</b>');
                htmlDoc = htmlDoc.replace(/\n/g, '<br/><br/>');
                
                infoContent.innerHTML = htmlDoc;
                infoPanel.classList.add('visible');
                
                // Render math equations using KaTeX if available
                if (window.renderMathInElement) {
                    window.renderMathInElement(infoContent, {
                        delimiters: [
                            {left: '$$', right: '$$', display: true},
                            {left: '$', right: '$', display: false}
                        ],
                        throwOnError: false
                    });
                }
            } else {
                infoPanel.classList.remove('visible');
            }
        } else {
            infoPanel.classList.remove('visible');
        }
    });
}
function updateGraph(graphData) {
    if (!window.visNetwork) {
        initGraph(graphData);
    } else {
        window.visNetwork.setData({
            nodes: new vis.DataSet(graphData.nodes),
            edges: new vis.DataSet(graphData.edges)
        });
    }
}

function renderSuggestions() {
    const list = document.getElementById('suggestions-list');
    if (!list) return;
    list.innerHTML = '';
    if (window.currentSuggestions.length === 0) {
        list.innerHTML = '<li>No suggestions currently.</li>';
    } else {
        window.currentSuggestions.forEach(s => {
            const li = document.createElement('li');
            li.style.marginBottom = '6px';
            li.textContent = s;
            list.appendChild(li);
        });
    }
}

function displayRustResults(data) {
    // Update Graph and Suggestions
    if (data.graph) {
        updateGraph(data.graph);
    }
    if (data.suggestions) {
        window.currentSuggestions = data.suggestions;
        renderSuggestions();
    }

    // Update the bottom status bar
    if (data.envelope_areas_m2) {
        document.getElementById('stat-area').textContent = data.envelope_areas_m2.total_floor.toFixed(1) + ' m²';
        if (data.envelope_areas_m2.exterior_perimeter_m !== undefined && document.getElementById('stat-perimeter')) {
            document.getElementById('stat-perimeter').textContent = data.envelope_areas_m2.exterior_perimeter_m.toFixed(1) + ' m';
        }
    }

    // Populate and show the floating Results Overlay card
    window.lastEnergyData = data;
    const overlay = document.getElementById('results-overlay');
    const content = document.getElementById('results-content');
    if (overlay && content) {
        let html = '';
        if (data.envelope_areas_m2) {
            html += `<div class="result-row" data-result-key="floor_area"><span class="result-label">Floor Area:</span><span class="result-value">${data.envelope_areas_m2.total_floor.toFixed(1)} m²</span></div>`;
        }
        if (data.tabula_u_values) {
            html += `<div class="result-row" data-result-key="u_wall"><span class="result-label">U-Wall:</span><span class="result-value">${data.tabula_u_values.wall_W_m2K.toFixed(2)} W/m²K</span></div>`;
            html += `<div class="result-row" data-result-key="u_window"><span class="result-label">U-Window:</span><span class="result-value">${data.tabula_u_values.window_W_m2K.toFixed(2)} W/m²K</span></div>`;
        }
        if (data.heating_demand) {
            html += `<div class="result-row" data-result-key="heating_demand"><span class="result-label">Heating Demand:</span><span class="result-value highlight">${data.heating_demand.specific_Q_H_nd_kWh_m2a.toFixed(1)} kWh/m²a</span></div>`;
        }
        if (data.heat_losses) {
            html += `<div class="result-row" data-result-key="heat_losses"><span class="result-label">Heat Losses:</span><span class="result-value">${data.heat_losses.Q_ht_kWh_a.toFixed(1)} kWh/a</span></div>`;
            if (data.heat_losses.transmission_loss_kWh_a) {
                html += `<div class="result-row" data-result-key="transmission_loss"><span class="result-label" style="padding-left:12px;">• Transmission Loss:</span><span class="result-value">${data.heat_losses.transmission_loss_kWh_a.toFixed(1)} kWh/a</span></div>`;
            }
            if (data.heat_losses.ventilation_loss_kWh_a) {
                html += `<div class="result-row" data-result-key="ventilation_loss"><span class="result-label" style="padding-left:12px;">• Ventilation Loss:</span><span class="result-value">${data.heat_losses.ventilation_loss_kWh_a.toFixed(1)} kWh/a</span></div>`;
            }
        }
        if (data.heat_gains) {
            if (typeof data.heat_gains.solar_gains_kWh_a === 'number') {
                html += `<div class="result-row" data-result-key="solar_gains"><span class="result-label">Solar Gains:</span><span class="result-value">${data.heat_gains.solar_gains_kWh_a.toFixed(1)} kWh/a</span></div>`;
            }
            if (typeof data.heat_gains.internal_gains_kWh_a === 'number') {
                html += `<div class="result-row" data-result-key="internal_gains"><span class="result-label" style="color: #38bdf8; font-weight: 600;">Internal Gains (DIN V 18599):</span><span class="result-value highlight" style="color: #38bdf8;">${data.heat_gains.internal_gains_kWh_a.toFixed(1)} kWh/a</span></div>`;
            }
        }
        if (data.final_energy) {
            html += `<div class="result-row" data-result-key="final_energy"><span class="result-label">Final Energy:</span><span class="result-value">${data.final_energy.specific_Q_final_kWh_m2a.toFixed(1)} kWh/m²a</span></div>`;
        }
        if (data.overheating) {
            const isExempt = data.overheating.exemption !== "Not Exempt";
            const passes = data.overheating.passes;
            const statusText = isExempt ? data.overheating.exemption : (passes ? "Pass" : "Fail");
            const textColor = (passes || isExempt) ? "#22c55e" : "#ef4444";
            html += `<div class="result-row" data-result-key="overheating"><span class="result-label" style="color: ${textColor}; font-weight: 600;">Summer Overheating:</span><span class="result-value highlight" style="color: ${textColor}; font-weight: 600;">${statusText} (S_v: ${data.overheating.s_vorh.toFixed(3)} / S_z: ${data.overheating.s_zul.toFixed(3)})</span></div>`;
        }
        content.innerHTML = html;
        overlay.classList.add('visible');
    }
    updateInsulationVisualizer(data);

    // If formula modal is currently active, update its live calculations
    const modal = document.getElementById('formula-modal');
    if (modal && modal.classList.contains('active')) {
        const key = modal.getAttribute('data-active-key');
        const config = RESULT_CONFIGS[key];
        const bodyEl = document.getElementById('formula-modal-body');
        if (config && bodyEl) {
            const liveContainer = bodyEl.querySelector('.live-explanation-container');
            if (liveContainer && config.calculateExplanation) {
                liveContainer.innerHTML = config.calculateExplanation(data);
                if (window.renderMathInElement) {
                    window.renderMathInElement(liveContainer, {
                        delimiters: [
                            {left: '$$', right: '$$', display: true},
                            {left: '$', right: '$', display: false}
                        ],
                        throwOnError: false
                    });
                }
            }
        }
    }
}

function updateInsulationVisualizer(data) {
    const breakdown = data.wall_insulation_breakdown;
    const visBaseU = document.getElementById('vis-base-u');
    const visIns = document.getElementById('vis-insulation');
    const visInsLabel = document.getElementById('vis-ins-label');
    const mathRBase = document.getElementById('math-r-base');
    const mathRIns = document.getElementById('math-r-ins');
    const mathUFinal = document.getElementById('math-u-final');

    if (!breakdown) return;

    if (visBaseU) {
        visBaseU.textContent = `U: ${breakdown.u_wall_base.toFixed(2)}`;
    }

    const hasOverride = document.getElementById('custom-wall-override').checked;
    if (hasOverride) {
        const thickness = parseFloat(document.getElementById('custom-wall-thickness').value) || 0.2;
        // Map thickness (0 to 0.5m) to width (0px to 140px)
        const maxWidth = 140;
        const maxThickness = 0.5;
        const widthPx = Math.round((thickness / maxThickness) * maxWidth);
        
        if (visIns) {
            visIns.style.width = `${widthPx}px`;
            if (widthPx > 0) {
                visIns.classList.add('has-width');
            } else {
                visIns.classList.remove('has-width');
            }
        }
        if (visInsLabel) {
            visInsLabel.textContent = `${Math.round(thickness * 100)} cm`;
        }
        if (mathRBase) mathRBase.textContent = breakdown.r_wall_base.toFixed(3) + ' m²K/W';
        if (mathRIns) mathRIns.textContent = breakdown.r_insulation.toFixed(3) + ' m²K/W';
        if (mathUFinal) mathUFinal.textContent = breakdown.u_wall_final.toFixed(3) + ' W/m²K';
    } else {
        if (visIns) {
            visIns.style.width = '0px';
            visIns.classList.remove('has-width');
        }
        if (visInsLabel) visInsLabel.textContent = '0 cm';
        if (mathRBase) mathRBase.textContent = '--';
        if (mathRIns) mathRIns.textContent = '--';
        if (mathUFinal) mathUFinal.textContent = '--';
    }
}


async function dispatchState() {
    try {
        if (!window.sketchpadRs) {
            let retries = 30; // 3 seconds max
            while (!window.sketchpadRs && !window.wasmInitPromise && retries > 0) {
                await new Promise(resolve => setTimeout(resolve, 100));
                retries--;
            }
            if (!window.sketchpadRs) {
                if (window.wasmInitPromise) {
                    await window.wasmInitPromise;
                } else {
                    logToConsole("WASM engine not ready yet.");
                    return;
                }
            }
        }
        const payload = getRustStatePayload();
        
        // Removed Graph integration

        const numZones = payload.ui_state && payload.ui_state.raw_zones ? payload.ui_state.raw_zones.length : 0;
        logToConsole(`Dispatching state to WASM... (${numZones} zones)`);
        
        const resultJson = window.sketchpadRs.update_state(JSON.stringify(payload));
        const data = JSON.parse(resultJson);
        if (data.status === 'success') {
            displayRustResults(data);
            logToConsole("WASM calculations updated successfully.");
        } else {
            logToConsole(`WASM calculation error: ${data.message}`);
        }
    } catch (err) {
        logToConsole(`Error in dispatchState: ${err.message}`);
    }
}
window.dispatchState = dispatchState;

async function handleUndo() {
    try {
        logToConsole("Undo clicked");
        if (!window.sketchpadRs) await window.wasmInitPromise;
        const resultJson = window.sketchpadRs.undo();
        const result = JSON.parse(resultJson);
        if (result.state) {
            syncUIFromState(result.state);
            if (result.energy && result.energy.status === 'success') {
                displayRustResults(result.energy);
            }
            logToConsole("Undo successful.");
        } else {
            logToConsole("No undo history available.");
        }
    } catch (err) {
        logToConsole(`Error during undo: ${err.message}`);
    }
}

async function handleRedo() {
    try {
        logToConsole("Redo clicked");
        if (!window.sketchpadRs) await window.wasmInitPromise;
        const resultJson = window.sketchpadRs.redo();
        const result = JSON.parse(resultJson);
        if (result.state) {
            syncUIFromState(result.state);
            if (result.energy && result.energy.status === 'success') {
                displayRustResults(result.energy);
            }
            logToConsole("Redo successful.");
        } else {
            logToConsole("No redo history available.");
        }
    } catch (err) {
        logToConsole(`Error during redo: ${err.message}`);
    }
}

// Hook into addZone, removeZone, geometry changes
const _origAddZone = addZone;
addZone = function(type, x, y, width, length) {
    const res = _origAddZone(type, x, y, width, length);
    dispatchState();
    return res;
};

const _origRemoveZone = removeZone;
removeZone = function(id) {
    _origRemoveZone(id);
    dispatchState();
};

function rebuildAndRetainSelection(targetId = null) {
    renderZoneList();
    render3DZones();
    updateStats();
    dispatchState();
    
    if (!targetId && selectedObject) {
        targetId = selectedObject.userData.type === 'window' ? selectedObject.userData.windowId : selectedObject.userData.zoneId;
    }

    if (targetId) {
        const newMesh = draggableObjects.find(obj => 
            (obj.userData.type === 'window' && obj.userData.windowId === targetId) ||
            (obj.userData.type === 'zone' && obj.userData.zoneId === targetId)
        );
        if (newMesh) {
            selectedObject = newMesh;
            translateControl.showZ = false;
            scaleControl.showZ = false;

            if (window.activeTransformMode === 'scale') {
                scaleControl.attach(selectedObject);
                translateControl.detach();
            } else {
                translateControl.attach(selectedObject);
                scaleControl.detach();
            }
        } else {
            translateControl.detach();
            scaleControl.detach();
            selectedObject = null; selectedWall = null; selectedWindowId = null; updateSelectionPanel();
        }
    }
    updateSelectionPanel();
}

function initCompass() {
    const rotInput = document.getElementById('building-rotation');
    const rotVal = document.getElementById('rotation-value');
    const compassNeedle = document.getElementById('compass-needle');
    const compassAngle = document.getElementById('compass-angle');
    
    if (rotInput) {
        rotInput.addEventListener('input', (e) => {
            const val = e.target.value;
            if (rotVal) rotVal.textContent = val + '°';
            if (compassNeedle) compassNeedle.setAttribute('transform', `rotate(${val}, 50, 50)`);
            if (compassAngle) compassAngle.textContent = val + '°';
            buildingRotationDeg = parseFloat(val);
            if (window.northArrow) {
                // update north arrow
                const rad = -buildingRotationDeg * Math.PI / 180;
                window.northArrow.setDirection(new THREE.Vector3(Math.sin(rad), Math.cos(rad), 0));
            }
            // dispatch state
            dispatchState();
        });
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// //#region Result Formulas & Calculations Modal
// ─────────────────────────────────────────────────────────────────────────────

const RESULT_CONFIGS = {
    floor_area: {
        title: "Floor Area ($A_{NGF}$)",
        formula: "The Net Floor Area is the total usable floor area of the building, calculated by summing the area of all 2D zones and multiplying by the number of stories:<br/>$$A_{NGF} = A_{2D} \\times n_{stories}$$",
        inputs: ["num-stories", "story-height"],
        calculateExplanation: (data) => {
            if (!data.envelope_areas_m2) return '';
            const total = data.envelope_areas_m2.total_floor;
            const num_stories = parseInt(document.getElementById('num-stories').value) || 1;
            const area_2d = total / num_stories;
            return `
                <div style="background: rgba(56, 189, 248, 0.05); border: 1px solid rgba(56, 189, 248, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live Calculation:</strong><br/>
                    $$A_{NGF} = ${area_2d.toFixed(1)}\\text{ m}^2 \\times ${num_stories} = ${total.toFixed(1)}\\text{ m}^2$$
                </div>
            `;
        }
    },
    u_wall: {
        title: "U-Wall (Thermal Transmittance of Walls)",
        formula: "The U-value measures heat loss through the wall. Lower values mean better insulation. Calculated as:<br/>$$U_{wall} = \\frac{1}{R_{si} + \\sum \\frac{d_i}{\\lambda_i} + R_{se}}$$ <br/> Where $R_{si} = 0.13\\text{ m}^2K/W$ and $R_{se} = 0.04\\text{ m}^2K/W$. By default, standard values are fetched from the TABULA database based on building age, typologies, and refurbishment scenario.",
        inputs: ["tabula-type", "tabula-year", "tabula-scenario", "custom-wall-mat", "custom-wall-thick"],
        calculateExplanation: (data) => {
            if (!data.tabula_u_values) return '';
            const u_wall = data.tabula_u_values.wall_W_m2K;
            let breakdownHtml = '';
            if (data.wall_insulation_breakdown) {
                const breakdown = data.wall_insulation_breakdown;
                breakdownHtml = `
                    <div style="margin-top: 8px; font-size: 0.85rem; color: #94a3b8;">
                        • Base Wall U-Value: ${breakdown.u_wall_base.toFixed(3)} W/m²K<br/>
                        • Combined U-Value: ${breakdown.u_wall_final.toFixed(3)} W/m²K
                    </div>
                `;
            }
            return `
                <div style="background: rgba(56, 189, 248, 0.05); border: 1px solid rgba(56, 189, 248, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live U-Wall:</strong> <span style="color:#38bdf8; font-weight:700;">${u_wall.toFixed(3)} W/m²K</span>
                    ${breakdownHtml}
                </div>
            `;
        }
    },
    u_window: {
        title: "U-Window (Thermal Transmittance of Windows)",
        formula: "The U-value of windows, determined by glazing and shutter control. If night shutters are automated or manual, the effective U-value is adjusted:<br/>$$U_{w,eff} = U_w \\cdot (1 - f_{sh}) + U_{w,sh} \\cdot f_{sh}$$<br/>Where $f_{sh}$ is the shutter usage fraction (dependent on automation class and climate region) and $U_{w,sh}$ represents the combined window+shutter resistance.",
        inputs: ["shutter-control", "tabula-year"],
        calculateExplanation: (data) => {
            if (!data.tabula_u_values) return '';
            const u_win = data.tabula_u_values.window_W_m2K;
            return `
                <div style="background: rgba(56, 189, 248, 0.05); border: 1px solid rgba(56, 189, 248, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live U-Window:</strong> <span style="color:#38bdf8; font-weight:700;">${u_win.toFixed(3)} W/m²K</span>
                </div>
            `;
        }
    },
    heating_demand: {
        title: "Heating Demand ($Q_{h}$)",
        formula: "The annual space heating demand of the building based on the DIN V 4108-6/DIN V 18599 monthly energy balance:<br/>$$Q_h = Q_l - \\eta \\cdot Q_g$$<br/>Where $Q_l$ is total losses (transmission + ventilation), $Q_g$ is total gains (solar + internal), and $\\eta$ is the gain utilization factor dependent on building thermal mass time constant $\\tau$.",
        inputs: ["tabula-type", "tabula-year", "tabula-scenario", "heating-system", "thermal-bridge", "ground-contact", "shutter-control", "climate-region", "usage-profile", "automation-class"],
        calculateExplanation: (data) => {
            if (!data.heating_demand) return '';
            const spec = data.heating_demand.specific_Q_H_nd_kWh_m2a;
            const q_h = data.heating_demand.Q_H_nd_kWh_a;
            return `
                <div style="background: rgba(16, 185, 129, 0.05); border: 1px solid rgba(16, 185, 129, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live Annual Heating Demand:</strong> <span style="color:#10b981; font-weight:700;">${q_h.toFixed(1)} kWh/a</span><br/>
                    <strong>Specific Demand:</strong> <span style="color:#10b981; font-weight:700;">${spec.toFixed(1)} kWh/m²a</span>
                </div>
            `;
        }
    },
    heat_losses: {
        title: "Total Heat Losses ($Q_l$)",
        formula: "The sum of heat lost through the building envelope via conduction (transmission) and air exchanges (ventilation):<br/>$$Q_l = Q_T + Q_V$$",
        inputs: ["thermal-bridge", "ground-contact", "climate-region", "air-tightness", "has-atd", "mech-supply", "mech-exhaust", "heat-recovery", "mech-hours"],
        calculateExplanation: (data) => {
            if (!data.heat_losses) return '';
            const q_l = data.heat_losses.Q_ht_kWh_a;
            const q_t = data.heat_losses.transmission_loss_kWh_a || 0;
            const q_v = data.heat_losses.ventilation_loss_kWh_a || 0;
            return `
                <div style="background: rgba(239, 68, 68, 0.05); border: 1px solid rgba(239, 68, 68, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live Total Losses ($Q_l$):</strong> <span style="color:#f87171; font-weight:700;">${q_l.toFixed(1)} kWh/a</span><br/>
                    <strong>Transmission Losses ($Q_T$):</strong> ${q_t.toFixed(1)} kWh/a<br/>
                    <strong>Ventilation Losses ($Q_V$):</strong> ${q_v.toFixed(1)} kWh/a
                </div>
            `;
        }
    },
    transmission_loss: {
        title: "Transmission Loss ($Q_T$)",
        formula: "Heat loss through walls, roofs, windows, and thermal bridges directly to the outside or ground:<br/>$$Q_T = [ H_{T,D} + H_{T,iu} \\cdot F_x + H_{T,WB} ] \\cdot (\\theta_i - \\theta_e) \\cdot t$$<br/>Where $H_{T,D}$ is direct transmission, $F_x$ is temperature correction factor, and $H_{T,WB}$ is the thermal bridge penalty.",
        inputs: ["thermal-bridge", "ground-contact", "climate-region"],
        calculateExplanation: (data) => {
            if (!data.heat_losses || !data.heat_losses.transmission_loss_kWh_a) return '';
            const q_t = data.heat_losses.transmission_loss_kWh_a;
            return `
                <div style="background: rgba(239, 68, 68, 0.05); border: 1px solid rgba(239, 68, 68, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live Transmission Losses ($Q_T$):</strong> <span style="color:#f87171; font-weight:700;">${q_t.toFixed(1)} kWh/a</span>
                </div>
            `;
        }
    },
    ventilation_loss: {
        title: "Ventilation Loss ($Q_V$)",
        formula: "Heat loss caused by building air change through window airing, infiltration, and mechanical ventilation systems:<br/>$$Q_V = [ H_{V,inf} + H_{V,win} + H_{V,mech} ] \\cdot (\\theta_i - \\theta_{V,supply}) \\cdot t$$",
        inputs: ["air-tightness", "has-atd", "mech-supply", "mech-exhaust", "heat-recovery", "mech-hours"],
        calculateExplanation: (data) => {
            if (!data.heat_losses || !data.heat_losses.ventilation_loss_kWh_a) return '';
            const q_v = data.heat_losses.ventilation_loss_kWh_a;
            return `
                <div style="background: rgba(239, 68, 68, 0.05); border: 1px solid rgba(239, 68, 68, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live Ventilation Losses ($Q_V$):</strong> <span style="color:#f87171; font-weight:700;">${q_v.toFixed(1)} kWh/a</span>
                </div>
            `;
        }
    },
    solar_gains: {
        title: "Solar Gains ($Q_S$)",
        formula: "Passive solar heat gains received through transparent building elements (windows):<br/>$$Q_S = \\sum I_j \\cdot A_{w,j} \\cdot g \\cdot F_F \\cdot F_C$$<br/>Where $I_j$ is solar radiation, $A_{w,j}$ is window area, $g$ is solar factor, $F_F$ is frame fraction, and $F_C$ is shading factor.",
        inputs: ["climate-region", "building-rotation"],
        calculateExplanation: (data) => {
            if (!data.heat_gains || typeof data.heat_gains.solar_gains_kWh_a !== 'number') return '';
            const q_s = data.heat_gains.solar_gains_kWh_a;
            return `
                <div style="background: rgba(56, 189, 248, 0.05); border: 1px solid rgba(56, 189, 248, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live Solar Heat Gains ($Q_S$):</strong> <span style="color:#38bdf8; font-weight:700;">${q_s.toFixed(1)} kWh/a</span>
                </div>
            `;
        }
    },
    internal_gains: {
        title: "Internal Gains ($Q_I$)",
        formula: "Heat generated inside the building by occupants, electrical equipment, and artificial lighting, depending on usage profile:<br/>$$Q_I = [ q_{occ} \\cdot N_{occ} + q_{equip} + q_{lighting} ] \\cdot t$$",
        inputs: ["usage-profile", "custom-occupants", "custom-equipment", "lighting-exhaust", "material-transport"],
        calculateExplanation: (data) => {
            if (!data.heat_gains || typeof data.heat_gains.internal_gains_kWh_a !== 'number') return '';
            const q_i = data.heat_gains.internal_gains_kWh_a;
            return `
                <div style="background: rgba(56, 189, 248, 0.05); border: 1px solid rgba(56, 189, 248, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live Internal Heat Gains ($Q_I$):</strong> <span style="color:#38bdf8; font-weight:700;">${q_i.toFixed(1)} kWh/a</span>
                </div>
            `;
        }
    },
    final_energy: {
        title: "Final Energy ($Q_{f}$)",
        formula: "The actual energy purchased for heating, factoring in the efficiency (SPF/COP or boiler losses) of the heating system:<br/>$$Q_{final} = \\frac{Q_h}{\\eta_{sys}}$$ <br/> Where the system efficiency $\\eta_{sys}$ depends on the heat generator type.",
        inputs: ["heating-system"],
        calculateExplanation: (data) => {
            if (!data.final_energy) return '';
            const q_f = data.final_energy.specific_Q_final_kWh_m2a;
            return `
                <div style="background: rgba(56, 189, 248, 0.05); border: 1px solid rgba(56, 189, 248, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Live Specific Final Energy:</strong> <span style="color:#38bdf8; font-weight:700;">${q_f.toFixed(1)} kWh/m²a</span>
                </div>
            `;
        }
    },
    overheating: {
        title: "Summer Overheating ($S_{vorh}$ vs $S_{zul}$)",
        formula: "Thermal protection against summer overheating according to DIN 4108-2. The existing solar entry factor $S_{vorh}$ must not exceed the maximum allowable solar entry factor $S_{zul}$:<br/>$$S_{vorh} = \\frac{\\sum A_i \\cdot g_{tot,i}}{A_{NGF,eff}} \\le S_{zul}$$<br/>Where $g_{tot} = g \\cdot F_C \\cdot F_S$. Certain rooms are exempt if windows are small or heavily shaded.",
        inputs: ["tabula-year", "shutter-control", "climate-region", "usage-profile", "air-tightness", "has-atd", "mech-supply"],
        calculateExplanation: (data) => {
            if (!data.overheating) return '';
            const sv = data.overheating.s_vorh;
            const sz = data.overheating.s_zul;
            const isExempt = data.overheating.exemption !== "Not Exempt";
            const status = isExempt ? `Exempt (${data.overheating.exemption})` : (data.overheating.passes ? "Pass" : "Fail");
            return `
                <div style="background: rgba(56, 189, 248, 0.05); border: 1px solid rgba(56, 189, 248, 0.2); padding: 12px; border-radius: 8px; margin: 12px 0;">
                    <strong>Status:</strong> <span style="font-weight:700;">${status}</span><br/>
                    <strong>Existing Solar Entry ($S_{vorh}$):</strong> ${sv.toFixed(4)}<br/>
                    <strong>Allowable Solar Entry ($S_{zul}$):</strong> ${sz.toFixed(4)}
                </div>
            `;
        }
    }
};

const FRIENDLY_LABELS = {
    'num-stories': 'Number of Stories',
    'story-height': 'Story Height (m)',
    'tabula-type': 'House Type',
    'tabula-year': 'Year of Build',
    'tabula-scenario': 'Refurbishment Status',
    'custom-wall-mat': 'Wall Material Override',
    'custom-wall-thick': 'Wall Thickness Override (m)',
    'shutter-control': 'Shutter Control',
    'heating-system': 'Heating System',
    'thermal-bridge': 'Thermal Bridge Category',
    'ground-contact': 'Ground Contact Type',
    'climate-region': 'Climate Region',
    'usage-profile': 'Usage Profile (DIN V 18599)',
    'automation-class': 'Automation Class',
    'air-tightness': 'Air Tightness Category',
    'has-atd': 'Has Air Transfer Devices (ATD)',
    'mech-supply': 'Mech. Supply (m³/h)',
    'mech-exhaust': 'Mech. Exhaust (m³/h)',
    'heat-recovery': 'Heat Recovery (%)',
    'mech-hours': 'Mech. Sys Hrs/Day',
    'lighting-exhaust': 'Lighting Exhaust Type',
    'material-transport': 'Material Transport',
    'custom-occupants': 'Occupants (Custom)',
    'custom-equipment': 'Equipment (Watts)',
    'building-rotation': 'Building Rotation (North Angle)'
};

function createModalInputControl(sourceId) {
    const sourceEl = document.getElementById(sourceId);
    if (!sourceEl) return '';
    
    let labelText = FRIENDLY_LABELS[sourceId] || sourceId;

    let controlHtml = '';
    if (sourceEl.tagName === 'SELECT') {
        let optionsHtml = '';
        Array.from(sourceEl.options).forEach(opt => {
            optionsHtml += `<option value="${opt.value}" ${opt.value === sourceEl.value ? 'selected' : ''}>${opt.text}</option>`;
        });
        controlHtml = `
            <div class="modal-form-row" style="margin-bottom: 12px;">
                <label style="display:block; margin-bottom:4px; font-weight:500; color:#94a3b8; font-size:0.85rem;">${labelText}</label>
                <select data-source-id="${sourceId}" style="width:100%; background:#1e293b; border:1px solid rgba(255,255,255,0.1); border-radius:6px; color:#e2e8f0; padding:6px 10px; font-size:0.85rem;">
                    ${optionsHtml}
                </select>
            </div>
        `;
    } else if (sourceEl.tagName === 'INPUT' && sourceEl.type === 'checkbox') {
        controlHtml = `
            <div class="modal-form-row" style="margin-bottom: 12px; display:flex; align-items:center; gap:8px;">
                <input type="checkbox" data-source-id="${sourceId}" ${sourceEl.checked ? 'checked' : ''} style="width:16px; height:16px; margin:0; cursor:pointer;">
                <label style="font-weight:500; color:#e2e8f0; font-size:0.85rem; cursor:pointer;">${labelText}</label>
            </div>
        `;
    } else if (sourceEl.tagName === 'INPUT' && (sourceEl.type === 'number' || sourceEl.type === 'range' || sourceEl.type === 'hidden')) {
        const minAttr = sourceEl.getAttribute('min') ? `min="${sourceEl.getAttribute('min')}"` : '';
        const maxAttr = sourceEl.getAttribute('max') ? `max="${sourceEl.getAttribute('max')}"` : '';
        const stepAttr = sourceEl.getAttribute('step') ? `step="${sourceEl.getAttribute('step')}"` : '';
        
        if (sourceEl.type === 'range') {
            controlHtml = `
                <div class="modal-form-row" style="margin-bottom: 12px;">
                    <div style="display:flex; justify-content:space-between; margin-bottom:4px;">
                        <label style="font-weight:500; color:#94a3b8; font-size:0.85rem;">${labelText}</label>
                        <span class="modal-range-val" style="color:#38bdf8; font-weight:600; font-size:0.85rem;">${sourceEl.value}°</span>
                    </div>
                    <input type="range" data-source-id="${sourceId}" value="${sourceEl.value}" ${minAttr} ${maxAttr} ${stepAttr} style="width:100%; cursor:pointer;">
                </div>
            `;
        } else {
            controlHtml = `
                <div class="modal-form-row" style="margin-bottom: 12px;">
                    <label style="display:block; margin-bottom:4px; font-weight:500; color:#94a3b8; font-size:0.85rem;">${labelText}</label>
                    <input type="number" data-source-id="${sourceId}" value="${sourceEl.value}" ${minAttr} ${maxAttr} ${stepAttr} style="width:100%; background:#1e293b; border:1px solid rgba(255,255,255,0.1); border-radius:6px; color:#e2e8f0; padding:6px 10px; font-size:0.85rem;">
                </div>
            `;
        }
    }
    return controlHtml;
}

function openFormulaModal(key) {
    const config = RESULT_CONFIGS[key];
    if (!config) return;

    const modal = document.getElementById('formula-modal');
    const titleEl = document.getElementById('formula-modal-title');
    const bodyEl = document.getElementById('formula-modal-body');

    if (!modal || !titleEl || !bodyEl) return;

    titleEl.innerHTML = config.title;

    let inputsHtml = '';
    if (config.inputs && config.inputs.length > 0) {
        inputsHtml += `<h4 style="color:#e2e8f0; margin:16px 0 8px 0; border-bottom:1px solid rgba(255,255,255,0.08); padding-bottom:4px; font-size:0.9rem;">Interactive Inputs</h4>`;
        config.inputs.forEach(id => {
            inputsHtml += createModalInputControl(id);
        });
    }

    bodyEl.innerHTML = `
        <div class="formula-section" style="margin-bottom: 16px;">
            <h4 style="color:#e2e8f0; margin-bottom:6px; font-size:0.9rem;">Calculation Method & Formula</h4>
            <div class="formula-description" style="color:#94a3b8; background:rgba(0,0,0,0.15); border-radius:8px; padding:12px; border:1px solid rgba(255,255,255,0.03);">
                ${config.formula}
            </div>
        </div>
        <div class="live-explanation-container">
            ${config.calculateExplanation ? config.calculateExplanation(window.lastEnergyData || {}) : ''}
        </div>
        <div class="modal-inputs-form" style="margin-top:16px;">
            ${inputsHtml}
        </div>
    `;

    bodyEl.querySelectorAll('[data-source-id]').forEach(control => {
        const sourceId = control.getAttribute('data-source-id');
        const sourceEl = document.getElementById(sourceId);
        if (!sourceEl) return;

        const eventName = (control.tagName === 'INPUT' && control.type === 'range') ? 'input' : 'change';

        control.addEventListener(eventName, () => {
            if (control.type === 'checkbox') {
                sourceEl.checked = control.checked;
            } else {
                sourceEl.value = control.value;
            }

            if (control.type === 'range') {
                const valDisplay = control.parentElement.querySelector('.modal-range-val');
                if (valDisplay) valDisplay.textContent = control.value + '°';
            }

            sourceEl.dispatchEvent(new Event('input'));
            sourceEl.dispatchEvent(new Event('change'));

            dispatchState().then(() => {
                const liveContainer = bodyEl.querySelector('.live-explanation-container');
                if (liveContainer && config.calculateExplanation) {
                    liveContainer.innerHTML = config.calculateExplanation(window.lastEnergyData || {});
                    if (window.renderMathInElement) {
                        window.renderMathInElement(liveContainer, {
                            delimiters: [
                                {left: '$$', right: '$$', display: true},
                                {left: '$', right: '$', display: false}
                            ],
                            throwOnError: false
                        });
                    }
                }
            });
        });
    });

    if (window.renderMathInElement) {
        window.renderMathInElement(bodyEl, {
            delimiters: [
                {left: '$$', right: '$$', display: true},
                {left: '$', right: '$', display: false}
            ],
            throwOnError: false
        });
    }

    modal.classList.add('active');
    modal.setAttribute('data-active-key', key);
}

const btnCloseFormula = document.getElementById('btn-close-formula');
const formulaModal = document.getElementById('formula-modal');
if (btnCloseFormula && formulaModal) {
    btnCloseFormula.addEventListener('click', () => {
        formulaModal.classList.remove('active');
        formulaModal.removeAttribute('data-active-key');
    });
    formulaModal.addEventListener('click', (e) => {
        if (e.target === formulaModal) {
            formulaModal.classList.remove('active');
            formulaModal.removeAttribute('data-active-key');
        }
    });
}

const resultsContent = document.getElementById('results-content');
if (resultsContent) {
    resultsContent.addEventListener('click', (e) => {
        const row = e.target.closest('.result-row');
        if (row) {
            const key = row.getAttribute('data-result-key');
            if (key) {
                openFormulaModal(key);
            }
        }
    });
}

// //#endregion

function initEventListeners() {
    document.getElementById('btn-add-zone').addEventListener('click', () => {
        const type = document.getElementById('zone-type').value;
        const width = parseFloat(document.getElementById('zone-width').value) || 5;
        const length = parseFloat(document.getElementById('zone-length').value) || 4;
        const x = parseFloat(document.getElementById('zone-x').value) || 0;
        const y = parseFloat(document.getElementById('zone-y').value) || 0;
        addZone(type, x, y, width, length);
        document.getElementById('zone-x').value = x + width;
        document.getElementById('zone-y').value = y;
    });

    const btnOrbit = document.getElementById('btn-mode-orbit');
    const btnSelect = document.getElementById('btn-mode-select');
    const btnTranslate = document.getElementById('btn-transform-translate');
    const btnScale = document.getElementById('btn-transform-scale');
    const divider = document.getElementById('toolbar-divider');
    
    window.activeTransformMode = 'translate';

    if (btnOrbit) {
        btnOrbit.addEventListener('click', () => {
            selectionModeActive = false;
            btnOrbit.classList.add('active');
            btnSelect.classList.remove('active');
            
            if (btnTranslate) btnTranslate.style.display = 'none';
            if (btnScale) btnScale.style.display = 'none';
            if (divider) divider.style.display = 'none';
            
            dragControls.enabled = true;
            controls.enabled = true;
            if (translateControl && translateControl.object) {
                translateControl.detach();
                scaleControl.detach();
            }
            selectedObject = null; selectedWall = null; selectedWindowId = null; updateSelectionPanel();
        });

        btnSelect.addEventListener('click', () => {
            selectionModeActive = true;
            btnSelect.classList.add('active');
            btnOrbit.classList.remove('active');
            
            if (btnTranslate) btnTranslate.style.display = 'block';
            if (btnScale) btnScale.style.display = 'block';
            if (divider) divider.style.display = 'block';
            
            dragControls.enabled = false;
            
            window.activeTransformMode = 'translate';
            if (btnTranslate) btnTranslate.classList.add('active');
            if (btnScale) btnScale.classList.remove('active');
            if (selectedObject) {
                translateControl.attach(selectedObject);
                scaleControl.detach();
            }
        });

        if (btnTranslate) {
            btnTranslate.addEventListener('click', () => {
                window.activeTransformMode = 'translate';
                btnTranslate.classList.add('active');
                btnScale.classList.remove('active');
                if (selectedObject) {
                    translateControl.attach(selectedObject);
                    scaleControl.detach();
                }
            });
        }

        if (btnScale) {
            btnScale.addEventListener('click', () => {
                window.activeTransformMode = 'scale';
                btnScale.classList.add('active');
                btnTranslate.classList.remove('active');
                if (selectedObject) {
                    scaleControl.attach(selectedObject);
                    translateControl.detach();
                }
            });
        }
    }

    const btnUndo = document.getElementById('btn-undo');
    if (btnUndo) btnUndo.addEventListener('click', handleUndo);
    
    const btnRedo = document.getElementById('btn-redo');
    if (btnRedo) btnRedo.addEventListener('click', handleRedo);

    ['tabula-type', 'tabula-year', 'tabula-scenario', 'num-stories', 'story-height', 'heating-system', 'usage-profile', 'thermal-bridge', 'ground-contact', 'shutter-control', 'climate-region', 'automation-class',
     'custom-wall-mat', 'custom-wall-thick', 'custom-roof-mat', 'custom-roof-thick', 'custom-floor-mat', 'custom-floor-thick',
     'air-tightness', 'has-atd', 'mech-supply', 'mech-exhaust', 'heat-recovery', 'mech-hours',
     'lighting-exhaust', 'material-transport', 'custom-occupants', 'custom-equipment'].forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            el.addEventListener('change', () => {
                render3DZones();
                dispatchState();
            });
            if (el.type === 'number' || el.type === 'range') {
                el.addEventListener('input', () => {
                    render3DZones();
                    dispatchState();
                });
            }
        }
    });

    const cwCheckbox = document.getElementById('custom-wall-override');
    const cwLambda = document.getElementById('custom-wall-lambda');
    const cwThickness = document.getElementById('custom-wall-thickness');
    const thicknessDisplay = document.getElementById('thickness-display');

    if (cwCheckbox) {
        cwCheckbox.addEventListener('change', (e) => {
            const checked = e.target.checked;
            cwLambda.disabled = !checked;
            cwThickness.disabled = !checked;
            dispatchState();
        });
    }
    
    if (cwLambda) cwLambda.addEventListener('change', dispatchState);
    if (cwThickness) {
        cwThickness.addEventListener('input', (e) => {
            const val_m = parseFloat(e.target.value) || 0;
            if (thicknessDisplay) {
                thicknessDisplay.innerText = Math.round(val_m * 100) + ' cm';
            }
        });
        cwThickness.addEventListener('change', dispatchState);
    }
    
    // Modal controls
    const btnOpenSimulator = document.getElementById('btn-open-simulator');
    const btnCloseSimulator = document.getElementById('btn-close-simulator');
    const simulatorModal = document.getElementById('simulator-modal');
    
    if (btnOpenSimulator) {
        btnOpenSimulator.addEventListener('click', () => {
            simulatorModal.classList.add('active');
        });
    }
    
    if (btnCloseSimulator) {
        btnCloseSimulator.addEventListener('click', () => {
            simulatorModal.classList.remove('active');
        });
    }

    const btnToggleGraph = document.getElementById('btn-toggle-graph');
    const graphContainer = document.getElementById('graph-container');
    const canvasContainer = document.getElementById('canvas-container');
    
    if (btnToggleGraph && graphContainer && canvasContainer) {
        btnToggleGraph.addEventListener('click', () => {
            if (graphContainer.style.display === 'none') {
                graphContainer.style.display = 'block';
                canvasContainer.style.display = 'none';
                btnToggleGraph.classList.add('active');
                if (window.visNetwork) window.visNetwork.fit();
            } else {
                graphContainer.style.display = 'none';
                canvasContainer.style.display = 'block';
                btnToggleGraph.classList.remove('active');
            }
        });
    }

    const btnGetSuggestions = document.getElementById('btn-get-suggestions');
    const btnCloseSuggestions = document.getElementById('btn-close-suggestions');
    const suggestionsModal = document.getElementById('suggestions-modal');

    if (btnGetSuggestions && suggestionsModal) {
        btnGetSuggestions.addEventListener('click', () => {
            suggestionsModal.classList.add('active');
            renderSuggestions();
        });
    }
    if (btnCloseSuggestions && suggestionsModal) {
        btnCloseSuggestions.addEventListener('click', () => {
            suggestionsModal.classList.remove('active');
        });
    }
}

// ── Bootstrap ───────────────────────────────────────────────────────────────
initThree();
initEventListeners();
initCompass();
