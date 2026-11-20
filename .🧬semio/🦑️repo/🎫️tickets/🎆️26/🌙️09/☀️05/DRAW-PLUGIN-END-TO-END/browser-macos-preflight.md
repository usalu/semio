# macOS Browser Preflight

On macOS arm64, the managed Playwright cache contains Chromium and headless shell revision 1248 with installation and dependency markers. The headless executable exists and has execute permission. The shell does not set PLAYWRIGHT_BROWSERS_PATH; the acceptance owner supplies the managed cache. Executable presence does not prove a launch.

The actual PDF oracle was run with `repo tools exec bun -- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔮️check.mjs`. It exited 0 with all five cases passing: same PDF, metadata-only change, edited rectangle, empty page, and malformed document. This exercised native canvas and PDF.js. The registered oracle owner remains a separate in-flight gate.

The current inferred serve target is `repo run @semio-tech/framework-os-dev:serve-draw-react-prototype`, port 6064, with the owner-provided catalog. Activation and graph preparation remain prerequisites to the live locale journeys. Existing acceptance does not assert Layers, Catalogue, or Inspection panels; their runtime navigation remains to be verified.

The real managed Chromium headless shell launched successfully with the acceptance software GPU flags via `repo tools exec bun`, navigated a data URL and returned the expected title. The first setup attempt failed before launch because of an incorrect guessed package path; the successful retry resolved Playwright from the actual browser module using `createRequire`. This is browser infrastructure evidence, not a Draw journey.

Current real entry: inferred `repo run @semio-tech/framework-os-dev:serve-draw-react-prototype`, port 6064; editor-owned activation handle 49492 remains in the shared graph scan, with no activation body or server yet. Registered oracle owner handle 88669 is also waiting for that graph. Fresh focused ShellSync owner handle 70963 started with `repo run @semio-tech/framework-renderer-react:test --parallel=1 -- long 🔄️ShellSync --testNamePattern=sync attach target`; admitted at 512 MB and planning graph, no test body yet.

Current source trace: inferred `repo run @semio-tech/framework-os-dev:serve-draw-react-prototype` routes `serve draw react prototype`. Serve requires a matching activation receipt including the host plugin, otherwise invokes activation; it uses the catalog’s fixed port 6064 and starts the local hub/Vite owner. Activation requires its session/projection, healthy prepared Draw component and font support. `repo run @semio-tech/plugin-registry:generate` depends on `repo:generator-inputs`. `repo run @semio-tech/draw-js:browser-acceptance -- --serve http://127.0.0.1:6064/ --locale en --output <fresh ticket generated directory>` (repeat de) depends on `workspace:deps-browsers`, owns a five-minute cancellation control and supplies managed Playwright cache. Existing activation 49492 remains sole activation.

ShellSync owner 70963 was revalidated still planning its distinct graph, with no test body, then cancelled by its own handle to release capacity for Draw on coordinator instruction; it exited 130 and released leases. Its earlier Flow declaration failure was repaired by the owner and did not recur during the retry. Activation 49492 and oracle 88669 remain untouched.

## Direct entry and graph scope

`repo ./✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts --help --lang=en` immediately fails with `Original parent Script process invocation required`; this child-only router cannot be directly launched as a valid substitute for the registered task. `repo run --help` describes `--exclude-dependencies` as excluding dependent tasks, not bypassing project graph planning. The registered oracle remains waiting on the shared graph; an HTTP probe of port 6064 still refused the connection.

The acceptance runner now checks the localized Artifact, Catalogue, and Inspection panels; it selects a real layer and waits for inspector content. Syntax and existing fixture schema checks pass. These new assertions await live server verification. The mistaken `browser-oracle` target handle 50837 was cancelled in planning; correct `browser-oracle-check` is handle 37522.
