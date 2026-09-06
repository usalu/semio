# Current Frontend Observation

On 2026-09-05, the previous in-app Browser tab 1 was no longer part of the browser session and the browser tab list was empty. The root created a fresh tab 2 at the existing local Shell URL `http://127.0.0.1:63310/`. Navigation timed out. A subsequent DOM snapshot was empty and the current error/warn log query returned an empty list. Independently, the local read-only listener check showed Bun PID 11915 still listening on 127.0.0.1:63310.

This is not a working frontend acceptance. The current narrow Space component producer is still running under the Home owner's dedicated cache; the two old full-Stdio producers were explicitly cancelled as superseded. The root will reuse the new browser tab after component materialization completes. No browser cookies, credentials, storage state or page internals were inspected.

## Current Listener and Browser Check

The later listener census no longer found63310. It found the existing OS dev Vite process96031 bound to127.0.0.1:6013 in the canonical dev package. Reusing the established Browser binding and tab2, navigation to6013 timed out on Page.navigate; the subsequent DOM request timed out on focus emulation. A separate bounded read-only HTTP probe also timed out after8005ms with zero response bytes/status000. This does not distinguish an overloaded host from a dev-server fault, and it provides no rendered UI or JSPI-in-Chromium acceptance. No other agent's process was restarted or terminated.

Space's latest narrow threads1 build is now terminal Cargo101 before describe, while Home's public-member run stopped at149 os-host-full workflow async-migration diagnostics before Home laws. The execution lane is repairing that exact compile boundary. The Node-hosted canonical browser guest is qualified separately in `📓️root-browser-wasi-activation.md`; its evidence must not be promoted into a real frontend claim.
