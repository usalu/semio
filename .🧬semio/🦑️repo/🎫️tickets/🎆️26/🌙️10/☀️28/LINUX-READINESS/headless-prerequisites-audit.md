# Headless Linux Browser Prerequisites

The existing rootless Source-overlay container is the validation host. No download, image rebuild, full setup or native search query was launched.

Initial actual inventory found `/usr/bin/Xvfb`. `glxinfo`, `eglinfo`, `xdpyinfo` and system Chromium were absent from PATH. DISPLAY was unset in this explicit Docker exec environment; the devcontainer DISPLAY setting does not automatically apply to this independent exec. The default Playwright browser cache directory `/home/vscode/.cache/ms-playwright` did not exist. Installed dependency generations include Playwright and playwright-core 1.64.1; JavaScript package presence alone does not establish a browser executable.

One actual prerequisite probe is pending under handle 14028, waiting for registered Docker tool memory admission (2048 MiB), with no Xvfb or browser child started yet. It owns a display selected by Xvfb `-displayfd`, a bounded X11 handshake and an EGL/GLES 16×16 pbuffer query. It records shutdown and socket/process absence. It also reads the installed Playwright Chromium executable path without installation. Execution has a 20-second display budget and a five-second EGL child budget, excluding admission wait. Logs are under `🗑️generated/headless-*`.

The three actual product E2E targets and their authored prepare/serve closures remain unexecuted. Neither Vulkan llvmpipe enumeration nor a Mesa EGL context proves Chromium ANGLE, map tiles or pixel behavior. Final probe results will be appended when its existing handle reaches terminal state.

## Terminal Actual Proof

Handle 14028 completed with transport/probe status 0 after ordinary Docker capacity admission; no prerequisite was installed. Xvfb PID 6254 selected display :0 using displayfd, with a 1280×720×24 screen and TCP disabled. The independently encoded X11 setup handshake was accepted at protocol version 11. The probe sent SIGTERM only to its own Xvfb child; Xvfb exited 0, its process was absent and its socket was absent. The 20-second execution budget did not expire.

The actual EGL probe failed before context creation: loading `libEGL.so.1` raised `OSError: cannot open shared object file: No such file or directory`. No EGL/GLES or Chromium ANGLE capability was established. This is a concrete missing runtime library for the intended direct EGL probe; it does not by itself prove that Chromium, which may bundle graphics libraries, would fail in the same way.

The installed Playwright 1.64.1 package resolved Chromium to `/home/vscode/.cache/ms-playwright/chromium-1248/chrome-linux64/chrome`. That executable did not exist. No browser launch or browser runtime test was possible under the requested no-download scope. Normal owned browser dependency setup remains required before all three E2E gates.

Final evidence is `🗑️generated/headless-prerequisites-proof.json`, independently parsed from the streamed host log. All probe processes have settled; no browser was started. No production source was changed for this investigation.

## Authoritative Managed Cache Follow-up

Read-only terminal68147 evaluated the actual tool-registry environment function and native repoToolCacheEnv helper. Both matched the configured path used by final probe79951. The managed browser cache root and exact Chromium1248 executable were absent. See browser-managed-cache-audit.md for exact authored routing and proof.
