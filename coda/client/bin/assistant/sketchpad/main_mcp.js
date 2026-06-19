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
    };

    if (document.getElementById('custom-wall-override').checked) {
        params.custom_wall_insulation = {
            thickness_m: parseFloat(document.getElementById('custom-wall-thickness').value) || 0.2,
            lambda: parseFloat(document.getElementById('custom-wall-lambda').value) || 0.035
        };
    } else {
        params.custom_wall_insulation = null;
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

function displayRustResults(data) {
    // Update the bottom status bar
    if (data.envelope_areas_m2) {
        document.getElementById('stat-area').textContent = data.envelope_areas_m2.total_floor.toFixed(1) + ' m²';
        if (data.envelope_areas_m2.exterior_perimeter_m !== undefined && document.getElementById('stat-perimeter')) {
            document.getElementById('stat-perimeter').textContent = data.envelope_areas_m2.exterior_perimeter_m.toFixed(1) + ' m';
        }
    }

    // Populate and show the floating Results Overlay card
    const overlay = document.getElementById('results-overlay');
    const content = document.getElementById('results-content');
    if (overlay && content) {
        let html = '';
        if (data.envelope_areas_m2) {
            html += `<div class="result-row"><span class="result-label">Floor Area:</span><span class="result-value">${data.envelope_areas_m2.total_floor.toFixed(1)} m²</span></div>`;
        }
        if (data.tabula_u_values) {
            html += `<div class="result-row"><span class="result-label">U-Wall:</span><span class="result-value">${data.tabula_u_values.wall_W_m2K.toFixed(2)} W/m²K</span></div>`;
            html += `<div class="result-row"><span class="result-label">U-Window:</span><span class="result-value">${data.tabula_u_values.window_W_m2K.toFixed(2)} W/m²K</span></div>`;
        }
        if (data.heating_demand) {
            html += `<div class="result-row"><span class="result-label">Heating Demand:</span><span class="result-value highlight">${data.heating_demand.specific_Q_H_nd_kWh_m2a.toFixed(1)} kWh/m²a</span></div>`;
        }
        if (data.heat_losses) {
            html += `<div class="result-row"><span class="result-label">Heat Losses:</span><span class="result-value">${data.heat_losses.Q_ht_kWh_a.toFixed(1)} kWh/a</span></div>`;
            if (data.heat_losses.transmission_loss_kWh_a) {
                html += `<div class="result-row"><span class="result-label">Transmission Loss:</span><span class="result-value">${data.heat_losses.transmission_loss_kWh_a.toFixed(1)} kWh/a</span></div>`;
            }
            if (data.heat_losses.ventilation_loss_kWh_a) {
                html += `<div class="result-row"><span class="result-label">Ventilation Loss:</span><span class="result-value">${data.heat_losses.ventilation_loss_kWh_a.toFixed(1)} kWh/a</span></div>`;
            }
        }
        if (data.heat_gains && typeof data.heat_gains.solar_gains_kWh_a === 'number') {
            html += `<div class="result-row"><span class="result-label">Solar Gains:</span><span class="result-value">${data.heat_gains.solar_gains_kWh_a.toFixed(1)} kWh/a</span></div>`;
        }
        if (data.final_energy) {
            html += `<div class="result-row"><span class="result-label">Final Energy:</span><span class="result-value">${data.final_energy.specific_Q_final_kWh_m2a.toFixed(1)} kWh/m²a</span></div>`;
        }
        content.innerHTML = html;
        overlay.classList.add('visible');
    }
    updateInsulationVisualizer(data);
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
            if (window.wasmInitPromise) await window.wasmInitPromise;
            else {
                logToConsole("WASM engine not ready yet.");
                return;
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
// Event Listeners & Init
// ─────────────────────────────────────────────────────────────────────────────

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

    ['tabula-type', 'tabula-year', 'tabula-scenario', 'num-stories', 'story-height', 'heating-system', 'usage-profile'].forEach(id => {
        document.getElementById(id).addEventListener('change', () => {
            render3DZones();
            dispatchState();
        });
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
}

// ── Bootstrap ───────────────────────────────────────────────────────────────
initThree();
initEventListeners();
initCompass();
