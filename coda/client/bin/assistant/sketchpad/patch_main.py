import re

with open('main_mcp.js', 'r') as f:
    code = f.read()

# 1. Add globals
globals_addition = """
let selectedWall = null;
let selectedWindowId = null;

window.addWindowToSelectedWall = function() {
    if (!selectedWall) return;
    const zone = zones.find(z => z.id === selectedWall.zoneId);
    if (!zone) return;
    if (!zone.windows) zone.windows = [];
    const winId = 'win_' + Date.now();
    zone.windows.push({
        id: winId,
        wall_id: selectedWall.wallId,
        u: 0,
        v: 0,
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
    selectedObject = null;
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
        title.textContent = `Selected: Window`;
        const zone = zones.find(z => z.windows && z.windows.find(w => w.id === selectedWindowId));
        content.innerHTML = `
            <div style="font-size: 0.8rem; color: #94a3b8; margin-bottom: 8px;">
                Move or scale the window in the 3D view.
            </div>
            <button class="btn-primary" style="background: #ef4444; color: white;" onclick="removeWindow('${zone.id}', '${selectedWindowId}')">Delete Window</button>
        `;
    } else if (selectedWall) {
        panel.style.display = 'block';
        title.textContent = `Selected: Wall (${selectedWall.wallId}) of ${selectedWall.zoneId}`;
        content.innerHTML = `
            <button class="btn-primary" onclick="addWindowToSelectedWall()">+ Add Window</button>
        `;
    } else if (selectedObject && selectedObject.userData.type === 'zone') {
        panel.style.display = 'block';
        title.textContent = `Selected: Zone (${selectedObject.userData.zoneId})`;
        content.innerHTML = `
            <div style="font-size: 0.8rem; color: #94a3b8;">
                Use the Gizmo to move or scale the entire zone. Click on a specific wall to add windows.
            </div>
        `;
    } else {
        panel.style.display = 'none';
    }
}
"""
code = code.replace("let selectionModeActive = false;", "let selectionModeActive = false;\n" + globals_addition)

# 2. Update TransformControls to local space
code = code.replace("scaleControl.setMode('scale');", "scaleControl.setMode('scale');\n    translateControl.setSpace('local');\n    scaleControl.setSpace('local');")

# 3. Update mouseUp events
old_mouseUp = """    translateControl.addEventListener('mouseUp', function () {
        if (selectedObject) {
            const zoneId = selectedObject.userData.zoneId;
            const zone = zones.find(z => z.id === zoneId);
            if (zone) {
                zone.geometry.x = Math.round((selectedObject.position.x - zone.geometry.width / 2) * 2) / 2;
                zone.geometry.y = Math.round((selectedObject.position.y - zone.geometry.length / 2) * 2) / 2;
                rebuildAndRetainSelection(zoneId);
            }
        }
    });
    scaleControl.addEventListener('mouseUp', function () {
        if (selectedObject) {
            const zoneId = selectedObject.userData.zoneId;
            const zone = zones.find(z => z.id === zoneId);
            if (zone) {
                const scaleX = selectedObject.scale.x;
                const scaleY = selectedObject.scale.y;
                zone.geometry.width = Math.round(zone.geometry.width * scaleX * 10) / 10;
                zone.geometry.length = Math.round(zone.geometry.length * scaleY * 10) / 10;
                rebuildAndRetainSelection(zoneId);
            }
        }
    });"""

new_mouseUp = """    translateControl.addEventListener('mouseUp', function () {
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
    });"""
code = code.replace(old_mouseUp, new_mouseUp)

# 4. Raycaster selection logic
old_ray = """        const intersects = raycaster.intersectObjects(draggableObjects, false);
        if (intersects.length > 0) {
            const object = intersects[0].object;
            if (selectedObject !== object) {
                selectedObject = object;
                if (window.activeTransformMode === 'scale') {
                    scaleControl.attach(selectedObject);
                    translateControl.detach();
                } else {
                    translateControl.attach(selectedObject);
                    scaleControl.detach();
                }
            }
        } else {
            // Clicked empty space
            if (translateControl.object || scaleControl.object) {
                translateControl.detach();
                scaleControl.detach();
                selectedObject = null;
            }
        }"""

new_ray = """        const intersects = raycaster.intersectObjects(draggableObjects, false);
        if (intersects.length > 0) {
            const object = intersects[0].object;
            const userData = object.userData;
            
            if (userData.type === 'window') {
                selectedWindowId = userData.windowId;
                selectedWall = null;
                selectedObject = object;
                
                translateControl.showZ = false;
                scaleControl.showZ = false;

                if (window.activeTransformMode === 'scale') {
                    scaleControl.attach(selectedObject);
                    translateControl.detach();
                } else {
                    translateControl.attach(selectedObject);
                    scaleControl.detach();
                }
            } else if (userData.type === 'zone') {
                const nx = Math.round(intersects[0].face.normal.x);
                const ny = Math.round(intersects[0].face.normal.y);
                const nz = Math.round(intersects[0].face.normal.z);
                let wallId = null;
                if (nx === 0 && ny === 1 && nz === 0) wallId = 'N';
                else if (nx === 0 && ny === -1 && nz === 0) wallId = 'S';
                else if (nx === 1 && ny === 0 && nz === 0) wallId = 'E';
                else if (nx === -1 && ny === 0 && nz === 0) wallId = 'W';

                if (wallId) {
                    selectedWall = { zoneId: userData.zoneId, wallId };
                    selectedWindowId = null;
                    selectedObject = null;
                    translateControl.detach();
                    scaleControl.detach();
                } else {
                    selectedWall = null;
                    selectedWindowId = null;
                    selectedObject = object;
                    translateControl.showZ = false;
                    scaleControl.showZ = false;
                    
                    if (window.activeTransformMode === 'scale') {
                        scaleControl.attach(selectedObject);
                        translateControl.detach();
                    } else {
                        translateControl.attach(selectedObject);
                        scaleControl.detach();
                    }
                }
            }
            updateSelectionPanel();
        } else {
            selectedObject = null;
            selectedWall = null;
            selectedWindowId = null;
            translateControl.detach();
            scaleControl.detach();
            updateSelectionPanel();
        }"""
code = code.replace(old_ray, new_ray)

# 5. addZone to include windows array
code = code.replace(
    "const zone = { id: `zone_${zoneCounter}`, type, geometry: { x, y, width, length } };",
    "const zone = { id: `zone_${zoneCounter}`, type, geometry: { x, y, width, length }, windows: [] };"
)

# 6. rendering windows and userData.type
code = code.replace(
    "boxMesh.userData = { zoneId: zone.id, storyIndex: s };",
    "boxMesh.userData = { type: 'zone', zoneId: zone.id, storyIndex: s };"
)

render_windows = """
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
            }
            
            wMesh.position.set(wx, wy, wz);
            wMesh.rotation.set(rotX, rotY, 0);
            
            draggableObjects.push(wMesh);
            buildingGroup.add(wMesh);
        });

"""
code = code.replace("buildingGroup.add(sprite);\n    });", "buildingGroup.add(sprite);\n" + render_windows + "    });")

# 7. update getRustStatePayload
old_rust = """    const rust_zones = zones.map(z => ({
        id: z.id,
        room_type: z.type || "Zone",
        width: z.geometry.width,
        length: z.geometry.length,
        x: z.geometry.x,
        y: z.geometry.y
    }));"""
new_rust = """    const rust_zones = zones.map(z => ({
        id: z.id,
        room_type: z.type || "Zone",
        width: z.geometry.width,
        length: z.geometry.length,
        x: z.geometry.x,
        y: z.geometry.y,
        windows: z.windows || []
    }));"""
code = code.replace(old_rust, new_rust)

# 8. syncUIFromState
old_sync = """    zones = state.zones.map(z => ({
        id: z.id,
        type: z.room_type,
        geometry: { width: z.width, length: z.length, x: z.x, y: z.y }
    }));"""
new_sync = """    zones = state.zones.map(z => ({
        id: z.id,
        type: z.room_type,
        geometry: { width: z.width, length: z.length, x: z.x, y: z.y },
        windows: z.windows || []
    }));"""
code = code.replace(old_sync, new_sync)

# 9. rebuildAndRetainSelection target tracking
old_rebuild = """function rebuildAndRetainSelection(zoneId) {
    renderZoneList();
    render3DZones();
    updateStats();
    dispatchState();
    const newMesh = draggableObjects.find(obj => obj.userData.zoneId === zoneId);
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
        selectedObject = null;
    }
}"""
new_rebuild = """function rebuildAndRetainSelection(targetId = null) {
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
            selectedObject = null;
        }
    }
    updateSelectionPanel();
}"""
code = code.replace(old_rebuild, new_rebuild)

# 10. clear modes
code = code.replace("selectedObject = null;", "selectedObject = null; selectedWall = null; selectedWindowId = null; updateSelectionPanel();")

with open('main_mcp.js', 'w') as f:
    f.write(code)

