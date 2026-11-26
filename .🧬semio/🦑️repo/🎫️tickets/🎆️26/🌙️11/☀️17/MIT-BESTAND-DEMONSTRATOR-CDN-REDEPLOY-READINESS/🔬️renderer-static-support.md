# Renderer support on a static Demonstrator origin

The first current frontend Generator replay reached its new Flow window and reported HTTP 404 for `renderer-modules/compiler-fonts/🧮️LibertinusMath-Regular.otf`. The private Vite diagnostic had deferred the production copy plugin and mounted the two actor directory trees. The Renderer support tree uses its own top-level path.

The production `semio-browser-artifacts` plugin already passes six separately owned Renderer roots from `demonstratorRuntimeAssetSources` to `copyBrowserArtifacts`. They deliver compiler fonts, canvas fonts, the icon pack and the three package-asset projections. The preserved complete production Vite artifact included all ten files. Actual static HTTP GETs for all ten matched the current managed source expectation for status, response length and SHA-256. Eight of these are font URLs.

The private builder now invokes the same production copier and owner validation on the six selected Renderer sources. After the previous Chromium and its origin exited, the existing diagnostic root received only these support files. The compiled renderer and Flow assets and Procedural overlay preserved their previously recorded hashes. The diagnostic origin returned ten exact HTTP 200s. `LibertinusMath-Regular.otf` is 588,848 bytes at SHA-256 `ded714b0d3808af527438ed7a85f16933133f66a12104c4a83db4cc248888011`.

The final release site preflight now serves the immutable captured Vite output and downloads every Renderer support URL from the current authored artifact inventory. Any missing or changed response fails release validation before Chromium. This uses the same static server as the acceptance launcher and leaves the official 96-case roster unchanged.
