# Linux OpenGL Backend Audit

No additional production package declaration is justified by the measured missing system EGL library. The authored Linux browser backend uses desktop OpenGL through ANGLE; the existing browser/Xvfb package closure already supplies that backing stack. Proposed explicit GL/EGL package additions and their new expectation were removed after the actual package transaction disproved the assumed absence of system GL. Existing bootstrap/Dockerfile behavior and existing assertions were retained.

## Authored Backend and System Boundary

Demonstrator and Play select `--use-angle=gl` on Linux and `--use-angle=metal` on macOS. Their SwiftShader overrides are explicit reduced-capability choices. The current upstream [ANGLE FunctionsGLX implementation](https://github.com/google/angle/blob/main/src/libANGLE/renderer/gl/glx/FunctionsGLX.cpp) loads `libGL.so.1`, resolves GLX functions and requires GLX 1.3 or newer. [ANGLE DisplayGLX](https://chromium.googlesource.com/angle/angle.git/+/master/src/libANGLE/renderer/gl/glx/DisplayGLX.cpp) opens the selected X display before initializing that backing renderer.

Bundled ANGLE EGL/GLES libraries do not replace the selected desktop GL backing library. Conversely, absence of the *system* EGL dispatcher does not establish a browser failure when Chromium ships its own ANGLE libraries. No browser executable is present, so this audit does not assert that this particular Chromium package successfully supplies its bundled side of the interface.

## Existing Linux Package Closure

| Environment | Confirmed existing dependency path |
|---|---|
| Ubuntu container | Actual apt output: libgl1, libglx-mesa0 and libgl1-mesa-dri were already installed at the current versions before diagnostic installation. |
| Fedora | Xvfb requires libglvnd-glx; libglvnd-glx requires mesa-libGL; mesa-libGL requires mesa-dri-drivers. |
| Arch | Xvfb depends on libglvnd; the authored browser row already includes mesa, whose dependencies include libglvnd. |
| openSUSE | Official RPM headers: Xvfb requires Mesa and libGL.so.1; libglvnd provides GL/EGL/GLES and requires Mesa-dri. |

Sources: [Fedora Xvfb dependencies](https://packages.fedoraproject.org/pkgs/xorg-x11-server/xorg-x11-server-Xvfb/fedora-45.html), [Fedora GLX dispatcher](https://packages.fedoraproject.org/pkgs/libglvnd/libglvnd-glx/fedora-45.html), [Fedora Mesa GL](https://packages.fedoraproject.org/pkgs/mesa/mesa-libGL/fedora-rawhide.html), [Arch Xvfb](https://archlinux.org/packages/extra/x86_64/xorg-server-xvfb/), [Arch Mesa](https://archlinux.org/packages/extra/x86_64/mesa/), [Arch dispatcher file list](https://archlinux.org/packages/extra/x86_64/libglvnd/files/). The [official openSUSE package index](https://download.opensuse.org/tumbleweed/repo/oss/x86_64/) and its read-only RPM headers supplied the openSUSE facts; bounded parsed requirements/provides are temporarily retained in `🗑️generated/opensuse-graphics-closure.json`. Fedora/Arch/openSUSE package installation was not executed.

The first proposed neutral test demanded explicit declarations and failed on apt-get libgl1. That expectation was stricter than the real dependency contract and was discarded together with the proposed package additions. It is not retained as evidence of a Chromium defect. The unchanged bootstrap toolchain suite subsequently passed 7 tests and 87 assertions, terminal88412. Bash syntax and LF checks passed. No production Source changes remain from this subtask.

## Actual Bounded Runtime Proof

Registered Docker execution retained the existing container and its private Source overlay. No image rebuild, source copy, Chromium download, full setup or native query occurred.

The authorized diagnostic apt transaction32283 completed0. It explicitly found libgl1, libglx-mesa0 and libgl1-mesa-dri already installed. Only three small diagnostic packages were added: libegl-mesa0, libegl1 and libgles2, 163 kB downloaded and 609 kB installed. The [Ubuntu EGL dispatcher metadata](https://packages.ubuntu.com/en/noble/arm64/libegl1) distinguishes the dispatcher and Mesa vendor library. This addition enabled the direct system EGL/GLES probe; it is not claimed as a browser repair.

Runtime probe79951 completed0 under its 20-second execution budget:

- Xvfb selected display :0; the independent X11 setup request was accepted at protocol11.
- EGL1.5 initialized with Mesa and created a current 16×16 pbuffer context; glGetString reported OpenGL ES3.2, Mesa25.2.8, llvmpipe LLVM20.1.2.
- A separate desktop GLX1.4 context became current and reported OpenGL4.5 Compatibility Profile with the same Mesa llvmpipe renderer.
- EGL emitted the expected DRI3 acceleration warning in this software-only Xvfb environment. No hardware acceleration is claimed.
- Owned Xvfb PID6534 exited0 after SIGTERM; its process and X11 socket were absent afterward. No timeout or forced kill occurred.

The actual devcontainer-configured PLAYWRIGHT_BROWSERS_PATH was applied for the final presence check. Installed Playwright1.64.1 resolves Chromium to `/workspaces/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium-1248/chrome-linux64/chrome`; that executable is absent. This corrects the earlier probe's use of the default home cache, which was also absent.

Full temporary proof is `🗑️generated/headless-graphics-proof.json`; system installation and probe output were streamed directly to `headless-install-graphics.log` and `headless-graphics.log`. No probe child remains live.

## Remaining Acceptance Boundary

System desktop GLX and direct system EGL/GLES now have actual software-renderer proof. Chromium ANGLE startup, browser WebGL/WebGPU behavior, map requests/pixels and all three owned product E2E closures remain unexecuted. Normal owned browser provisioning must supply the pinned browser before those gates. The llvmpipe result does not establish GPU hardware or map acceptance.
