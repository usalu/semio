# React Map GPU Recovery

## Existing Device Support

The React map uses the shared browser Wasm `MapSession` and Vello's compute-based WebGPU renderer. Adapter admission precedes binding the WebGPU context to the canvas. The shared session reconfigures an outdated surface and skips timeouts or occluded frames. A lost surface or a validation error is returned as a render failure. The NodeGraph Canvas2D path is specific to graph scenes. The current map renderer requires a browser and GPU that can provide a WebGPU adapter.

Safari 26 introduced WebGPU on iOS, iPadOS and macOS. Chrome's WebGPU support depends on browser, OS and GPU, and requires a secure context. A phone opening a developer machine's plain HTTP LAN URL does not get the secure-context treatment of that machine's loopback URL. [WebKit Safari 26](https://webkit.org/blog/17333/webkit-features-in-safari-26-0/) · [Chrome WebGPU troubleshooting](https://developer.chrome.com/docs/web-platform/webgpu/troubleshooting-tips) · [Vello renderer families](https://github.com/linebender/vello)

## Recovery

The map pane shows the localized Canvas unavailable alert and Retry button after a rejected Wasm load, failed adapter attachment, startup exceeding 15 seconds or a failed render frame. The failed attempt releases the renderer, tile requests and resize observer. Retry opens a new Wasm session on a fresh canvas, rebinds pointer handling, and restores the live map camera when available. A late attachment belongs to the retired canvas. Device loss is reported when the session next attempts to render a frame.

The developer browser owner supplies the Wasm bindings port and maps its generated surface package through Vite. Storybook installs the same port before rendering its host stories. Bare Bun resolution of the generated package is outside that browser delivery path. Runtime asset failures are logged by the shared loader and can remove labels and icons while allowing the map session to start.

## Verification

The four mounted recovery cases failed before the implementation, including two unhandled startup rejections. All 46 React TiledMapHost tests then passed through the checked-in renderer Vitest configuration. The recovery fixture specifies the startup deadline, locale output and camera coordinates. The UI test adapter verifies the live alert, localized retry button, restored camera, exactly-once cleanup and pointer dispatch after replacing the canvas. The tests drive simulated Wasm sessions. Physical browser and GPU verification uses the separate device acceptance run.

The final recovery suite passed all four cases after matching the shared Retry button icon. A focused TypeScript check of the map source and recovery test reported zero diagnostics. The package-wide check reported 30 other diagnostics outside TiledMapHost.
