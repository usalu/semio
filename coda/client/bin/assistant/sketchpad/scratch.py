import re

with open("main_mcp.js", "r") as f:
    content = f.read()

# 1. Orthographic Camera
cam_old = """    camera = new THREE.PerspectiveCamera(60, rect.width / rect.height, 0.1, 1000);
    camera.position.set(12, 12, 10);
    camera.up.set(0, 0, 1);"""
cam_new = """    const aspect = rect.width / rect.height;
    const frustumSize = 30;
    camera = new THREE.OrthographicCamera((frustumSize * aspect) / -2, (frustumSize * aspect) / 2, frustumSize / 2, frustumSize / -2, -1000, 1000);
    camera.position.set(20, 20, 20); // Isometric perspective
    camera.up.set(0, 0, 1);"""
content = content.replace(cam_old, cam_new)

# 2. Resize listener
resize_old = """    // Resize
    window.addEventListener('resize', () => {
        const r = container.getBoundingClientRect();
        camera.aspect = r.width / r.height;
        camera.updateProjectionMatrix();
        renderer.setSize(r.width, r.height);
    });"""
resize_new = """    // Resize
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
    });"""
content = content.replace(resize_old, resize_new)

# 3. Mode Toggles & Attach
# We need to change rebuildAndRetainSelection to only attach the active one
attach_old = """        if (newMesh) {
            selectedObject = newMesh;
            translateControl.attach(selectedObject);
            scaleControl.attach(selectedObject);
        } else {"""
attach_new = """        if (newMesh) {
            selectedObject = newMesh;
            if (window.activeTransformMode === 'scale') {
                scaleControl.attach(selectedObject);
                translateControl.detach();
            } else {
                translateControl.attach(selectedObject);
                scaleControl.detach();
            }
        } else {"""
content = content.replace(attach_old, attach_new)

ptr_old = """            if (selectedObject !== object) {
                selectedObject = object;
                translateControl.attach(selectedObject);
                scaleControl.attach(selectedObject);
            }"""
ptr_new = """            if (selectedObject !== object) {
                selectedObject = object;
                if (window.activeTransformMode === 'scale') {
                    scaleControl.attach(selectedObject);
                    translateControl.detach();
                } else {
                    translateControl.attach(selectedObject);
                    scaleControl.detach();
                }
            }"""
content = content.replace(ptr_old, ptr_new)

ui_old = """    // Toolbar Listeners
    const btnOrbit = document.getElementById('btn-mode-orbit');
    const btnSelect = document.getElementById('btn-mode-select');

    if (btnOrbit) {
        btnOrbit.addEventListener('click', () => {
            selectionModeActive = false;
            btnOrbit.classList.add('active');
            btnSelect.classList.remove('active');
            
            dragControls.enabled = true;
            controls.enabled = true;
            if (translateControl && translateControl.object) {
                translateControl.detach();
                scaleControl.detach();
            }
            selectedObject = null;
        });

        btnSelect.addEventListener('click', () => {
            selectionModeActive = true;
            btnSelect.classList.add('active');
            btnOrbit.classList.remove('active');
            
            dragControls.enabled = false;
        });
    }"""
ui_new = """    // Toolbar Listeners
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
            selectedObject = null;
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
    }"""
content = content.replace(ui_old, ui_new)

# 4. ViewHelper Monkeypatch for Z-up quaternions
vh_old = """    // Setup ViewHelper
    viewHelper = new ViewHelper(camera, renderer.domElement);
    viewHelper.center = controls.target;

    container.addEventListener('pointerup', (event) => {
        viewHelper.handleClick(event);
    });"""
vh_new = """    // Setup ViewHelper
    viewHelper = new ViewHelper(camera, renderer.domElement);
    viewHelper.center = controls.target;

    // Monkeypatch ViewHelper to force Z-up Target Quaternions to prevent locking
    const originalHandleClick = viewHelper.handleClick.bind(viewHelper);
    viewHelper.handleClick = function(event) {
        const wasAnimating = this.animating;
        const result = originalHandleClick(event);
        if (this.animating && !wasAnimating) {
            // It successfully started an animation. Overwrite the target quaternion to be Z-up!
            // Depending on where it moved the camera, we look at center with Z-up.
            const targetPos = this.targetPosition;
            const dummyCam = camera.clone();
            dummyCam.position.copy(controls.target).add(targetPos.clone().multiplyScalar(camera.position.distanceTo(controls.target)));
            dummyCam.up.set(0, 0, 1);
            dummyCam.lookAt(controls.target);
            this.targetQuaternion.copy(dummyCam.quaternion);
        }
        return result;
    };

    container.addEventListener('pointerup', (event) => {
        viewHelper.handleClick(event);
    });"""
content = content.replace(vh_old, vh_new)

with open("main_mcp.js", "w") as f:
    f.write(content)
