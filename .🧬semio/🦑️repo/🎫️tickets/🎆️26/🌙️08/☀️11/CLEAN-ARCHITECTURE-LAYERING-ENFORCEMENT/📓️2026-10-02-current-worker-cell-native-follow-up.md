# Current WorkerCell Native Follow-Up

The prior full-before production inputs remain in the explicit-admission report. The current portable before run actually passed two laws and failed one stale source-name law: 13 assertions,119ms Bun,24.3s uncached Nx. Its earlier invocation without caller-owned artifact output refused before laws; no semantic verdict is inferred from that invocation.

The existing lower Drawing command PID50446 was observed live and then absent; its terminal result is not available in this execution lane. Kernel/UI/Renderer native consumers remain held for the DSL source frontier.

## Full Original Portable Test Input

SHA256 `60f39360cad8a2e77df933ecdd7bc9857e90cc3dfa223a831e07783677ee3560`

```typescript
import { test, expect } from "bun:test";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { createAdmittedWorkerCell, createWorkerCell } from "../🟦️.ts";
const require = createRequire(import.meta.url);
const memoize: (factory: (owner: string) => State) => ((owner: string) => State) = require("lodash/memoize");
type State = { locale: string; retained: string[]; constructions: number };
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
const root = resolve(import.meta.dir, "../../../../../../../../../../../../..");
const interpreter = resolve(import.meta.dir, "../../🎯️targets/🧊️wgpu/🦀️.rs");

test("closed independent lifecycle corpus retains exact admission and thread identities", () => {
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8")));
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, schemaVersion: 2 })).toBe(false);
  const actual = [];
  for (const row of fixture.cases) {
    const cell = createAdmittedWorkerCell<State>();
    const ready = new Set<string>();
    const reference = memoize(() => ({ locale: "", retained: [], constructions: 1 }));
    const observations = [], oracle = [];
    for (const operation of row.operations) {
      if (operation.action === "admit") {
        cell.admit(operation.owner, () => ({ locale: operation.locale, retained: [], constructions: 1 }), state => { state.locale = operation.locale; });
        ready.add(operation.owner); reference(operation.owner).locale = operation.locale;
      } else if (operation.action === "retain") {
        cell.read(operation.owner)!.retained.push(operation.value); reference(operation.owner).retained.push(operation.value);
      }
      const state = cell.read(operation.owner), expected = ready.has(operation.owner) ? reference(operation.owner) : undefined;
      observations.push(state ? { owner: operation.owner, state: "ready", ...structuredClone(state) } : { owner: operation.owner, state: "unadmitted" });
      oracle.push(expected ? { owner: operation.owner, state: "ready", ...structuredClone(expected) } : { owner: operation.owner, state: "unadmitted" });
    }
    expect(observations).toEqual(row.expected); expect(oracle).toEqual(row.expected); actual.push({ id: row.id, observations });
  }
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true }); writeFileSync(resolve(output, "worker-cell-lifecycle.json"), JSON.stringify(actual, null, 2));
});

test("nonDefault construction is lazy and scoped rather than implicit", () => {
  let calls = 0; const cell = createWorkerCell(() => ({ secret: ++calls }));
  expect(calls).toBe(0); expect(cell.read("a")).toBe(cell.read("a"));
  expect(cell.read("b").secret).toBe(2); expect(cell.read("a").secret).toBe(1);
});

test("actual Rust worker source requires owned factories and explicit locale admission", () => {
  const source = readFileSync(interpreter, "utf8");
  expect(source).not.toMatch(/impl<T: Default|test_worker_cell<T: Default/);
  expect(source).toContain("initialize: fn() -> T");
  expect(source).toContain("pub(crate) fn admit_ui_locale");
  expect(source).toContain('expect("retained UI requires explicit locale admission")');
});

```

## Current Production Inputs and Exact Changes

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs
Original SHA256 `e28354c3a8f13a67875a80052e8ed9d30a807bae8a04eba63a93768e58eb2a1e`; current SHA256 `2b8f500270449388037c3c64981e88c21e31615c66491215b9c9750406ff3370`.
```diff
--- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs (captured before)
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs (current)
@@ -30,6 +30,7 @@
 /// on a thread of its own, so no law meets another law's engine, documents or chrome registry — in parallel or in
 /// sequence — and the renderer suite is one usable gate instead of an order-dependent one.
 pub(crate) struct WorkerCell<T> {
+    initialize: fn() -> T,
     #[cfg(not(test))]
     inner: std::sync::OnceLock<Mutex<T>>,
     #[cfg(test)]
@@ -37,11 +38,11 @@
 }
 
 impl<T> WorkerCell<T> {
-    pub(crate) const fn new() -> Self {
+    pub(crate) const fn new(initialize: fn() -> T) -> Self {
         #[cfg(not(test))]
-        return Self { inner: std::sync::OnceLock::new() };
+        return Self { initialize, inner: std::sync::OnceLock::new() };
         #[cfg(test)]
-        return Self { inner: std::marker::PhantomData };
+        return Self { initialize, inner: std::marker::PhantomData };
     }
 }
 
@@ -51,20 +52,20 @@
     static TEST_WORKER_CELLS: std::cell::RefCell<std::collections::HashMap<usize, &'static dyn std::any::Any>> = std::cell::RefCell::new(std::collections::HashMap::new());
 }
 
-/// 🧪️ The calling test thread's own instance of the worker-cell state at `key`, created from `T::default()` on first use.
-/// Every renderer `WorkerCell` (this module's, the scenes' and the engine canvas') resolves through it under `cfg(test)`.
+/// 🧪️ The calling test thread's own instance of the worker-cell state at `key`, built by the cell's `initialize` on first
+/// use. Every renderer `WorkerCell` (this module's, the scenes' and the engine canvas') resolves through it under `cfg(test)`.
 #[cfg(test)]
-pub(crate) fn test_worker_cell<T: Default + 'static>(key: usize) -> &'static Mutex<T> {
-    let state = TEST_WORKER_CELLS.with(|cells| *cells.borrow_mut().entry(key).or_insert_with(|| Box::leak(Box::new(Mutex::new(T::default()))) as &'static dyn std::any::Any));
+pub(crate) fn test_worker_cell<T: 'static>(key: usize, initialize: impl FnOnce() -> T) -> &'static Mutex<T> {
+    let state = TEST_WORKER_CELLS.with(|cells| *cells.borrow_mut().entry(key).or_insert_with(|| Box::leak(Box::new(Mutex::new(initialize()))) as &'static dyn std::any::Any));
     state.downcast_ref::<Mutex<T>>().expect("one worker cell holds one state type")
 }
 
-impl<T: Default + 'static> WorkerCell<T> {
+impl<T: 'static> WorkerCell<T> {
     pub(crate) fn state(&self) -> &Mutex<T> {
         #[cfg(not(test))]
-        return self.inner.get_or_init(|| Mutex::new(T::default()));
+        return self.inner.get_or_init(|| Mutex::new((self.initialize)()));
         #[cfg(test)]
-        return test_worker_cell::<T>(std::ptr::from_ref(self).addr());
+        return test_worker_cell::<T>(std::ptr::from_ref(self).addr(), self.initialize);
     }
 
     /// 🔒️ Crate-visible like `state`: `🐚️Shell/🎯️targets/🧊️wgpu`'s `with_chrome_prefs` drives this
@@ -316,7 +317,71 @@
  * instead of reading from (or clobbering) a second, independent one. See
  * `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w3-interpreter-cutover.md`'s "CRITICAL FINDING"
  * for the original gap and the follow-up ticket work that closed it. */
-static UI_ENGINE: WorkerCell<ui_wgpu::wgpu::Ui> = WorkerCell::new();
+static UI_ENGINE: UiEngineCell = UiEngineCell::new();
+
+/// 🚫️ The named refusal of a retained UI engine access before the shell resolved its locale.
+pub(crate) const UI_ENGINE_LOCALE_UNRESOLVED: &str = "ui.engine.locale-unresolved: the retained UI engine is used before the shell resolved its locale";
+
+/// 🧪️ The locale a law's engine starts in when the law builds no shell — a test fixture, never a production default.
+#[cfg(test)]
+pub(crate) const TEST_UI_ENGINE_LOCALE: semio_framework_ui_locale::Locale = semio_framework_ui_locale::Locale::En;
+
+/// 🌐️ The retained UI engine cell. There is no default language, so the engine exists only once the shell resolved its
+/// locale ([`install_ui_engine_locale`]); every access before that is refused by name ([`UI_ENGINE_LOCALE_UNRESOLVED`]).
+pub(crate) struct UiEngineCell(WorkerCell<Option<ui_wgpu::wgpu::Ui>>);
+
+/// 🔐️ The held engine: dereferences to the `Ui`, refusing by name while none was installed.
+pub(crate) struct UiEngineGuard<'a>(MutexGuard<'a, Option<ui_wgpu::wgpu::Ui>>);
+
+impl std::ops::Deref for UiEngineGuard<'_> {
+    type Target = ui_wgpu::wgpu::Ui;
+
+    fn deref(&self) -> &Self::Target {
+        self.0.as_ref().expect(UI_ENGINE_LOCALE_UNRESOLVED)
+    }
+}
+
+impl std::ops::DerefMut for UiEngineGuard<'_> {
+    fn deref_mut(&mut self) -> &mut Self::Target {
+        self.0.as_mut().expect(UI_ENGINE_LOCALE_UNRESOLVED)
+    }
+}
+
+/// 🌱️ The engine before any locale: none in production; a law's fixture locale under `cfg(test)`.
+fn initial_ui_engine() -> Option<ui_wgpu::wgpu::Ui> {
+    #[cfg(test)]
+    return Some(ui_wgpu::wgpu::Ui::new(TEST_UI_ENGINE_LOCALE));
+    #[cfg(not(test))]
+    None
+}
+
+impl UiEngineCell {
+    const fn new() -> Self {
+        Self(WorkerCell::new(initial_ui_engine))
+    }
+
+    pub(crate) fn borrow(&self) -> UiEngineGuard<'_> {
+        UiEngineGuard(self.0.borrow())
+    }
+
+    pub(crate) fn borrow_mut(&self) -> UiEngineGuard<'_> {
+        self.borrow()
+    }
+
+    pub(crate) fn with<R>(&self, apply: impl FnOnce(&Self) -> R) -> R {
+        apply(self)
+    }
+}
+
+/// 🌐️ Installs the retained UI engine in the shell's resolved locale — the first resolution builds it, every later one
+/// (a `setLocale`, a preferences load) re-sets the language of its own chrome.
+pub(crate) fn install_ui_engine_locale(locale: semio_framework_ui_locale::Locale) {
+    let mut engine = UI_ENGINE.0.borrow();
+    match engine.as_mut() {
+        Some(ui) => ui.set_locale(locale),
+        None => *engine = Some(ui_wgpu::wgpu::Ui::new(locale)),
+    }
+}
 
 #[derive(Clone, Debug, PartialEq, Eq)]
 struct UiDocumentCloseOwner {
@@ -348,7 +413,7 @@
     }
 }
 
-static UI_DOCUMENT_CLOSE_QUEUE: WorkerCell<UiDocumentCloseQueue> = WorkerCell::new();
+static UI_DOCUMENT_CLOSE_QUEUE: WorkerCell<UiDocumentCloseQueue> = WorkerCell::new(Default::default);
 
 /// 🧹️ Retains the exact mounted document generation that a closed window must retire.
 pub(crate) fn request_ui_document_close(window_id: &str) -> bool {
@@ -596,7 +661,7 @@
     }
 }
 
-static SCENE_INTENTS: WorkerCell<SceneIntentQueue> = WorkerCell::new();
+static SCENE_INTENTS: WorkerCell<SceneIntentQueue> = WorkerCell::new(Default::default);
 pub(crate) const SCENE_POINTER_OWNER_CAPACITY: usize = 16;
 
 #[derive(Clone)]
@@ -666,7 +731,7 @@
     slots: Vec<ScenePointerOwner>,
 }
 
-static SCENE_POINTER_OWNERS: WorkerCell<ScenePointerOwners> = WorkerCell::new();
+static SCENE_POINTER_OWNERS: WorkerCell<ScenePointerOwners> = WorkerCell::new(Default::default);
 
 /// 🪪️ Captures the exact retained scene identity published by one document node.
 /// 🪟️ Every windowed tree container the retained surface `window_id` presents, measured against its scroll viewport
@@ -846,7 +911,7 @@
 /// 👆️ Last-seen `(pointer_down, pointer_button)` per `window_id`, so `dispatch_pointer_events` can
 /// detect Down/Up edges from `InputState`'s per-frame aggregate.
 #[cfg(test)]
-static POINTER_EDGE_STATE: WorkerCell<std::collections::HashMap<String, (bool, i16)>> = WorkerCell::new();
+static POINTER_EDGE_STATE: WorkerCell<std::collections::HashMap<String, (bool, i16)>> = WorkerCell::new(Default::default);
 
 /** 🖇️ Public hook for the sibling `w3-shell-input-cutover` workstream (region `shell::ShellInput`,
  * which this ticket must not touch): routes a fully-formed `ui_wgpu::wgpu::UiEvent` (built from raw
@@ -4039,14 +4104,14 @@
 //#endregion RetainedEngineCutover
 
 //#region UiImageLoading
-static UI_IMAGE_FETCH_MISS: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new();
-static UI_IMAGE_LAST_URL: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new();
-static UI_IMAGE_URL_CACHE: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new();
-static UI_IMAGE_SIZES: WorkerCell<std::collections::HashMap<String, (u32, u32)>> = WorkerCell::new();
+static UI_IMAGE_FETCH_MISS: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new(Default::default);
+static UI_IMAGE_LAST_URL: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new(Default::default);
+static UI_IMAGE_URL_CACHE: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new(Default::default);
+static UI_IMAGE_SIZES: WorkerCell<std::collections::HashMap<String, (u32, u32)>> = WorkerCell::new(Default::default);
 /// 🚫️ Per image id, the inline source the raster authority refused as the image's own ([`RasterUploadRefusal::Invalid`]) — kept, not
 /// re-offered each paint; the image paints its fallback until its source changes.
-static UI_IMAGE_REFUSED: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new();
-static UI_IMAGE_ASSET_FAULT: WorkerCell<Option<WorldAssetFault>> = WorkerCell::new();
+static UI_IMAGE_REFUSED: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new(Default::default);
+static UI_IMAGE_ASSET_FAULT: WorkerCell<Option<WorldAssetFault>> = WorkerCell::new(Default::default);
 #[cfg(test)]
 thread_local! {
     static INLINE_SVG_PARSE_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
@@ -5896,13 +5961,13 @@
     parked: Option<String>,
 }
 
-static CHROME_LEDGER: WorkerCell<ChromeLedger> = WorkerCell::new();
+static CHROME_LEDGER: WorkerCell<ChromeLedger> = WorkerCell::new(Default::default);
 
 /// 🎥️ The LIVE orbit of every world surface that painted this frame, keyed by surface id — the
 /// diagnostics twin of [`CHROME_LEDGER`], and for the same reason: `dumpMeshStats` reaches only
 /// `UI_ENGINE`, while the orbit lives on the shell's `world3d_states`, so the shell has to hand it
 /// over at the one point per frame where it holds both the surface id and the state.
-static WORLD_CAMERA_LEDGER: WorkerCell<std::collections::BTreeMap<String, Value>> = WorkerCell::new();
+static WORLD_CAMERA_LEDGER: WorkerCell<std::collections::BTreeMap<String, Value>> = WorkerCell::new(Default::default);
 
 /** 🎥️ Records ONE world surface's live orbit. Called once per surface per painted frame from
  * `🎞️Scenes/🎯️targets/🧊️wgpu`'s world3d render, gated on runtime diagnostics exactly as the chrome
@@ -6004,7 +6069,7 @@
     nodes: Vec<ui_contract::AccessibilityProjectionNode>,
 }
 
-static CHROME_ACCESSIBILITY: WorkerCell<ChromeAccessibilityPublication> = WorkerCell::new();
+static CHROME_ACCESSIBILITY: WorkerCell<ChromeAccessibilityPublication> = WorkerCell::new(Default::default);
 
 const ACCESSIBILITY_VISIBLE_WINDOW_CAPACITY: usize = 256;
 
@@ -6014,7 +6079,7 @@
     staging: Vec<String>,
 }
 
-static ACCESSIBILITY_VISIBLE_WINDOWS: WorkerCell<AccessibilityVisibleWindows> = WorkerCell::new();
+static ACCESSIBILITY_VISIBLE_WINDOWS: WorkerCell<AccessibilityVisibleWindows> = WorkerCell::new(Default::default);
 
 pub(crate) fn staged_retained_clock_surfaces() -> Vec<(String, ui_wgpu::wgpu::UiSurfaceToken)> {
     let visible = ACCESSIBILITY_VISIBLE_WINDOWS.borrow().staging.clone();

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs
Original SHA256 `ee69357455423f5cebadd8792f242df6c8881b3fa094390e11d46b5a25a8d398`; current SHA256 `ddc4136e73d4a6a911e0f0672d51a94c58d4b480602dfcd98b80661229f9d800`.
```diff
--- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs (captured before)
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs (current)
@@ -1558,7 +1558,7 @@
         #[cfg(not(test))]
         let state = self.0.get_or_init(|| Mutex::new(RefCell::new(T::default())));
         #[cfg(test)]
-        let state = crate::interpreter::test_worker_cell::<RefCell<T>>(std::ptr::from_ref(self).addr());
+        let state = crate::interpreter::test_worker_cell::<RefCell<T>>(std::ptr::from_ref(self).addr(), || RefCell::new(T::default()));
         let guard = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
         f(&guard)
     }

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs
Original SHA256 `281ea29a9ec6fbedcc777038c14901e241fa31117bc99c9b1d2f5dcd2e0d9e22`; current SHA256 `ae5d0c13f6469797d4d7a8d1e6ff9058591741ca501204a1c6ef5c87b47661c8`.
```diff
--- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs (captured before)
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs (current)
@@ -954,7 +954,7 @@
     }
 }
 
-static STAGED_ENGINE_SCENES: WorkerCell<StagedEngineScenes> = WorkerCell::new();
+static STAGED_ENGINE_SCENES: WorkerCell<StagedEngineScenes> = WorkerCell::new(Default::default);
 
 /// 📦️ One turn of the staged-paint → frame-packet transfer, driven from the frame build's own
 /// `EngineTransfer` phase. Returns `true` when nothing is left staged.
@@ -1985,9 +1985,9 @@
         && cache.tool_run_trace_window_id.is_none()
 }
 
-static MAP_TILE_ASSET_FAULT: WorkerCell<Option<WorldAssetFault>> = WorkerCell::new();
-
-static ENGINE_SURFACES: WorkerCell<EngineSurfaceRegistry> = WorkerCell::new();
+static MAP_TILE_ASSET_FAULT: WorkerCell<Option<WorldAssetFault>> = WorkerCell::new(Default::default);
+
+static ENGINE_SURFACES: WorkerCell<EngineSurfaceRegistry> = WorkerCell::new(Default::default);
 
 #[derive(Clone, Debug, Default, PartialEq)]
 struct TutorialSurfaceGeometry {
@@ -2409,7 +2409,7 @@
     }
 }
 
-static ATTACHED_SURFACES: WorkerCell<AttachedSurfaceRegistry> = WorkerCell::new();
+static ATTACHED_SURFACES: WorkerCell<AttachedSurfaceRegistry> = WorkerCell::new(Default::default);
 
 /// 🧩️ Drains the surfaces attached since the last drain, for the shell to mirror into its bespoke
 /// pointer state maps. Taking rather than reading keeps the drain a record of what this frame
@@ -6003,7 +6003,7 @@
 
 /// 🧮️ Marquee gestures are per SURFACE and never outlive one press/release pair, so a plain map
 /// keyed by surface id is the whole bound — the same shape `scenes`' own `map_marquee_points` uses.
-static PAINT2D_MARQUEES: WorkerCell<HashMap<String, Paint2dMarquee>> = WorkerCell::new();
+static PAINT2D_MARQUEES: WorkerCell<HashMap<String, Paint2dMarquee>> = WorkerCell::new(Default::default);
 
 fn with_paint2d_marquee<R>(surface_id: &str, f: impl FnOnce(&mut Paint2dMarquee) -> R) -> R {
     PAINT2D_MARQUEES.with(|cell| {

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
Original SHA256 `ec2f94eaf71118efc53c6f2981a3511a5522f006ade425481b0374b232a1755b`; current SHA256 `28b9c0ea0ad898e4930541874bd8816dab4665ed9a5e0e8c7af4e7963afc2e14`.
```diff
--- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs (captured before)
+++ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs (current)
@@ -7019,6 +7019,7 @@
     /// every hop is one `fetch` the page makes on this shell's behalf — so the transport is a unit
     /// value and only the cancellation root has to be minted.
     pub fn new(plugins: Vec<ProgramBridgeEntry>, plugin_filter: String, locale: Locale, terminology: Terminology) -> Self {
+        crate::interpreter::install_ui_engine_locale(locale);
         let space_mode = is_space_mode(&plugin_filter);
         #[cfg(not(target_arch = "wasm32"))]
         let (directory_transport, directory_cancel): (ShellDirectoryTransport, CancelToken) = {
@@ -12038,6 +12039,7 @@
                         if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                             let changed = self.locale_id != value;
                             self.locale_id = value.to_string();
+                            crate::interpreter::install_ui_engine_locale(self.active_locale());
                             if let Some(status) = self.plugin_fault_status() {
                                 self.error = Some(status);
                             }
@@ -27143,6 +27145,7 @@
                 let custom_themes = custom_themes_from(&preferences);
                 self.appearance_id = resolve_appearance_id(&preferences);
                 self.locale_id = resolve_locale_id(env_lock("SEMIO_LOCKED_LOCALE"), &preferences, self.active_locale());
+                crate::interpreter::install_ui_engine_locale(self.active_locale());
                 self.terminology_id = env_lock("SEMIO_LOCKED_TERMINOLOGY").or(preferences.terminology).unwrap_or_else(|| self.active_terminology().as_str().to_string());
                 self.driver_id = preferences.driver_id.unwrap_or_else(|| "default".to_string());
                 self.chrome_build.preferences.custom_drivers = preferences.custom_drivers;
@@ -31997,7 +32000,7 @@
 }
 
 #[cfg(not(target_arch = "wasm32"))]
-static CHROME_PREFS: crate::interpreter::WorkerCell<Option<ChromePrefsState>> = crate::interpreter::WorkerCell::new();
+static CHROME_PREFS: crate::interpreter::WorkerCell<Option<ChromePrefsState>> = crate::interpreter::WorkerCell::new(Default::default);
 
 fn default_compute_worker_count() -> u32 {
     std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(1)
@@ -32998,6 +33001,7 @@
         let preferences = read_ui_preferences();
         self.appearance_id = resolve_appearance_id(&preferences);
         self.locale_id = resolve_locale_id(locks.locale.clone(), &preferences, self.active_locale());
+        crate::interpreter::install_ui_engine_locale(self.active_locale());
         self.terminology_id = locks.terminology.clone().or(preferences.terminology).unwrap_or_else(|| self.active_terminology().as_str().to_string());
         self.driver_id = preferences.driver_id.unwrap_or_else(|| "default".to_string());
         self.chrome_build.preferences = with_chrome_prefs(|preferences| preferences.clone());
@@ -34456,7 +34460,7 @@
 }
 
 #[cfg(not(any(target_arch = "wasm32", test)))]
-static CHROME_CONTROL_NAMES: crate::interpreter::WorkerCell<BTreeMap<String, ChromeControlPresentation>> = crate::interpreter::WorkerCell::new();
+static CHROME_CONTROL_NAMES: crate::interpreter::WorkerCell<BTreeMap<String, ChromeControlPresentation>> = crate::interpreter::WorkerCell::new(Default::default);
 
 fn with_chrome_control_names<R>(f: impl FnOnce(&mut BTreeMap<String, ChromeControlPresentation>) -> R) -> R {
     #[cfg(any(target_arch = "wasm32", test))]

```

### 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs
Original SHA256 `4fa9df4b317b491d7a6126f0e57190b37b6f8daced02945b29850cb92dd5d8de`; current SHA256 `f0e5d24d44d1b8f7aa7103a664ce84004a3f1a38b238142e95957de779188d69`.
```diff
--- 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs (captured before)
+++ 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs (current)
@@ -1207,6 +1207,11 @@
         }
     }
 
+    /// 🌐️ Re-sets the language the engine's own chrome strings resolve in — the host's resolved locale changed.
+    pub fn set_locale(&mut self, locale: crate::wgpu::Locale) {
+        self.shell.set_locale(locale);
+    }
+
     pub fn set_theme(&mut self, theme: Theme) {
         let layout_changed = theme_layout_identity(&self.theme) != theme_layout_identity(&theme);
         if !layout_changed && self.theme_propagation.is_none() {

```

## Additive Native Registration

Actual strengthened before run failed only missing native registration; no native compiler ran. The additive suffix below is the sole new Interpreter edit owned by this lane. Removing that exact suffix restores its before SHA256 `2b8f500270449388037c3c64981e88c21e31615c66491215b9c9750406ff3370`. Original native law bodies are unchanged.

```rust

#[cfg(test)]
#[path = "../../🧵️worker-cell/🧪️tests/🦀️.rs"]
mod worker_cell_tests;
```

## Current Portable Receipt

The complete registered `framework-renderer-wgpu:test-worker-cell` route is terminal GREEN:4laws,27assertions,zero failures,101ms Bun,29.9s uncached Nx. It preserves both closed ten-observation lifecycle rows, strict Ajv schema admission and lodash memoization outputs; new source laws inspect actual construction, unresolved refusal, Shell boot and all three preference forwards, Ui.set_locale sole existing-Shell operation, scene RefCell/poison handling, and actual native suite mounting. The previous strengthened run was genuine3pass/1 fail/26assertions before adding the mount.

Four additive native laws are authored and mounted. No current native Renderer runtime is claimed; the full unchanged target is held through DSL source/Cargo mutation readiness and requires normal preparation/locked metadata at release.

The observed separate Drawing PID69364 stdout belongs to the UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O ticket (`drawing-borrowed-component-native-current.log`). It is an independent selected SQLite check; it cannot grant this ticket's complete48-host native scope. Earlier PID50446 terminal remains unavailable. macOS comm fields truncate tool paths; live audits must parse the first full command token, not comm suffixes.

## Executed Value Binary Current Input Closure

Actual unchanged original complete Value target passed113/113/zero skipped in5.717s runtime,32.09s compilation,1m23s uncached Nx. The retained actual Nextest metadata selects binary `semio_framework_value-c311e41dcdbd7aef`. Its actual rustc `.d` has52 checksum-bearing owned inputs; all52 current physical lengths and first-party TypeScript BLAKE3 digests match,zero mismatches. Exact new lower type rows:

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🏷️type/🦀️.rs`: 3940bytes, BLAKE3 `a0404fae0b66edd52a9c2251a0a9d698eaf6c69beffaf694e2a06c9439530a51`.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🏷️type/🧪️tests/🦀️.rs`: 2326bytes, BLAKE3 `7fb03a6ea09b9750da069ff03c8c64195077ab7b275995cb642e1694e90a4974`.
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🏷️type/🧫️fixtures/🔣️.json`: 18773bytes, BLAKE3 `697ae0dbe323296dd1fc6c7a51e77c52432ebc3b17e0a0a42a16a883652d69e4`.

This proves the actually executed package own type implementation/native cases/closed fixture binding. Dependency fingerprints, Graph/Neural/higher runtime and product-absence remain separate obligations. Root is preparing additive neutral decode-law mounts; the113-law binary cannot prove those later additions.

## Current Additive Value Compile and Metadata Refusal

Root SOURCE-READY1 fifteen own inputs were freshly SHA256/length verified:zero mismatches. The unchanged entire `value-rs:test` replay with original long policy compiled the actual added lower laws successfully in1m06s,then ordinary unchanged Nextest all-features workspace metadata refused the in-progress generalDSL↔Replication cycle. WholeNx1m24s/exit1. No native runtime/discovered115count is claimed. Actual nextest artifacts directory is `value-decode-native-artifacts/semio-nextest-Y0n5pv`.

The real compiled library-test `.d` has58 checksum-bearing owned inputs;all 58 current physical lengths andfirst-party BLAKE3 match,zero mismatches. Seven decode owner rows include actual new law source and allrelevant fixtures. These are compilation/current-input receipts only; they do not prove execution. Root keeps the higher duplicate mount until actual complete new lower runtime. Coherent DSL Cargo source-ready,normal preparation/metadata and same entire Value replay remain required.

## Explicit Worker Caller Output Launch Gap

Actual generated automatic worker launch supplies no required `SEMIO_TEST_ARTIFACT_DIR`; current authored seed has no worker command orreserved900.05816entry. Earlieractual prelaw refusal validates thisgap. The explicit seed must supply caller-owned ticketoutput andthe registered portable law must compare Bun JSONC with the existing third-party jsonc-parser before checking the actual selected launch. Current full before seed iscaptured below to preserve unrelated concurrent additions.

### .vscode/🧩️launch.seed.jsonc

SHA256 `d1cce5a0fe8c3535d590dd1adb98c08a1c2c7061d07b42646d648170c97df79d`

```jsonc
{
  "version": "0.2.0",
  "configurations": [
    {"name":"⚖️gate🧱️block🖐️5d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-5d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.696}},
    {"name":"⚖️gate🧱️block🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.695}},
    {"name":"⚖️gate🧱️block◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.694}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.693}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.692}},

    {"name":"⚖️test-snapshot-sqlite📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.691}},
    {"name":"⚖️test-snapshot-sqlite-source📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.69109999999995}},
    {"name":"⚖️test-snapshot-sqlite-native📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6912}},
    {"name":"⚖️test-quick📖️pdf🌊️structural🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-pdf-rs:test --skip-nx-cache -- quick --lib lossless_structural_flow_law_bachelor_thesis_snapshot_mutation_diff_io_and_inverse --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6904}},
    {"name":"⚖️sqlite-source🧱️🖐️5d","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-5d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.688}},
    {"name":"⚖️sqlite-public🧱️🖐️5d","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-5d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.689}},
    {"name":"⚖️test-quick📕️xlsx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-xlsx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6901}},
    {"name":"⚖️test-quick📜️docx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-docx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6902}},
    {"name":"⚖️test-quick📽️pptx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-pptx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6903}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.685}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.686}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.687}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.684}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.683}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.682}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.681}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.679}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.678}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.677}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.674}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.675}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.676}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.673}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.672}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.671}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.670}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.669}},
    {"name":"⚖️gate📸️remodel📸️remodeling🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/remodel-remodeling-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.668}},
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.65
      }
    },
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.65099999999995
      }
    },
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite-source --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.652
      }
    },
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.623}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.624}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.625}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🏗️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.626}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🔍️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.627}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🧪️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.628}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.629}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.63}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.631}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.632}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.633}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.634}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.635}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.636}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.637}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.638}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.639}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.64}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.641}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.642}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.643}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.644}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.645}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.646}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.653}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.654}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.655}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.656}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.657}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.658}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.659}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.66}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.661}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.662}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.663}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.664}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.665}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.666}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.667}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.619}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.62}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.621}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-js:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.622}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.596}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.597}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.598}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio:test","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.599}},

    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.592}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59299999999996}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.594}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59499999999997}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.588}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.589}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59000000000003}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.591}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.584}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.585}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.586}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.587}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58099999999996}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.582}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58299999999997}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.576}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.577}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.578}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly:test","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.579}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.572}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.573}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.574}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.575}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.568}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.569}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.57}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.571}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.564}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.565}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.566}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.567}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.56}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.561}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.562}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.563}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.55}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.551}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.552}},
    {
      "name": "⚖️gate🪐️space🏠️home🪶️sqlite🟦️public",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/space-home:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.553
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.6
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.601
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60200000000003
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🏭️public",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.603
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.604
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60499999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.606
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60699999999997
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.608
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.609
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60999999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.611
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.61199999999997
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.613
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.614
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.615
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.616
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.61699999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.618
      }
    },
    {"name": "⚖️test-controlled-encoding🌱️value🦀️", "type": "node-terminal", "request": "launch", "command": "bun nx run @semio-tech/value-rs:test-controlled-encoding", "cwd": "${workspaceFolder}", "presentation": {"group": "4_gate", "order": 900.033208}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.543}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.540}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.541}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.542}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.530}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.531}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.532}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.523}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.52}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.521}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.522}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.5}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.501}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.502}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.503}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.504}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.505}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.506}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.507}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.508}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-js:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.494}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.491}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.492}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.493}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.465}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.466}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.467}},
    {"name":"⚖️build📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.468}},
    {"name":"⚖️check📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.469}},
    {"name":"⚖️test📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.47}},
    {"name":"⚖️gate📕️norm🧬️contract🌱️bytes🖥️bounded","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-artifact-contract-rs:test-byte-property","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.471}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.453}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.454}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.455}},
    {"name":"⚖️build📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.456}},
    {"name":"⚖️check📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.457}},
    {"name":"⚖️test📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.458}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.447}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.448}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.449}},
    {"name":"⚖️build📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.45}},
    {"name":"⚖️check📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.451}},
    {"name":"⚖️test📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.452}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.417}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.418}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.419}},
    {"name":"⚖️build📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.420}},
    {"name":"⚖️check📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.421}},
    {"name":"⚖️test📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.422}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.435}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.436}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.437}},
    {"name":"⚖️build📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.438}},
    {"name":"⚖️check📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.439}},
    {"name":"⚖️test📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.440}},

    {
      "name": "⚖️build📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.414
      }
    },
    {
      "name": "⚖️check📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.415
      }
    },
    {
      "name": "⚖️test📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.416
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.411
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.412
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.413
      }
    },
    {
      "name": "⚖️build📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.402
      }
    },
    {
      "name": "⚖️check📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.403
      }
    },
    {
      "name": "⚖️test📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.404
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.399
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.4
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.401
      }
    },
    {
      "name": "⚖️build📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.396
      }
    },
    {
      "name": "⚖️check📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.397
      }
    },
    {
      "name": "⚖️test📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.398
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.393
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.394
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.395
      }
    },
    {
      "name": "⚖️build📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.39
      }
    },
    {
      "name": "⚖️check📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.391
      }
    },
    {
      "name": "⚖️test📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.392
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.387
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.388
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.389
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.381
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.382
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.383
      }
    },
    {
      "name": "⚖️build📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.384
      }
    },
    {
      "name": "⚖️check📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.385
      }
    },
    {
      "name": "⚖️test📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.386
      }
    },
    {
      "name": "🦑️Repo 📋️native owner command 🧪️policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-native-owner-command-policy --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05751
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🦀️native 🧭️command dispatch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-command-dispatch --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05752
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🦀️native 🧭️command types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:check-command-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05753
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "◻️2D 🧮️compute 🧪️portable schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:test-schema --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05754
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "◻️2D 🧮️compute 🧭️contract types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:check-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05755
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05787
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-2d:test-compute --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057871
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute🔏️keys",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-2d:test-compute-keys --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057872
      }
    },
    {
      "name": "🌐️UI locale 🦀️native owner laws",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05808
      }
    },
    {
      "name": "🧪️test🖱️ui🧊️feature-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-feature-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-ui-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05812
      }
    },
    {
      "name": "⚖️check↔️paged-history-stack🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:paged-history-stack-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1814
      }
    },
    {
      "name": "⚖️check↔️paged-history-stack🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:paged-history-stack-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1815
      }
    },
    {
      "name": "⚖️check🔐️hub-auth-client🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1811
      }
    },
    {
      "name": "⚖️test🔐️hub-auth-client🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1812
      }
    },
    {
      "name": "⚖️test-source🔐️hub-auth-client🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:test-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1813
      }
    },
    {
      "name": "⚖️check👁️source-watch🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:source-watch-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.181
      }
    },
    {
      "name": "⚖️verify🔗️puzzle-browser-contribution🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-puzzle-composition-tests:verify-browser-contribution",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1809
      }
    },
    {
      "name": "⚖️check📍️distribution-output🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:distribution-output-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1808
      }
    },
    {
      "name": "⚖️test-component-owners📇️registry🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-component-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1807
      }
    },
    {
      "name": "⚖️test-artifact-kind🧰️framework🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-rs:test-artifact-kind",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05665
      }
    },
    {
      "name": "⚖️test-artifact-kind-source🧰️framework🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-rs:test-artifact-kind-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.056651
      }
    },
    {
      "name": "⚖️check🖍️draw-guest-instance🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-plugin:guest-instance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2647
      }
    },
    {
      "name": "⚖️check🧩️hub-component-codecs🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:component-codec-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2648
      }
    },
    {
      "name": "⚖️check⏱️hub-gis-codec-budget🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:component-codec-budget-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2649
      }
    },
    {
      "name": "⚖️check🧫️host-fixture🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host-fixture:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2638
      }
    },
    {
      "name": "⚖️build🧫️host-fixture-component🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host-fixture:component-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2639
      }
    },
    {
      "name": "⚖️check🧫️host-owned-instance🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host:owned-instance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.264
      }
    },
    {
      "name": "⚖️check🗒️note-snapshot-guest🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-snapshot-guest",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2641
      }
    },
    {
      "name": "⚖️check🗒️note-sqlite-snapshot-guest🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-sqlite-snapshot-guest",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2642
      }
    },
    {
      "name": "⚖️check🗺️gis-mcp-inference-source🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-bridge-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2643
      }
    },
    {
      "name": "⚖️check🗺️gis-mcp-inference-process🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-bridge-process-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2644
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-stdio-dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-stdio-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.8
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-stdio-release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-stdio-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.81
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-http-dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-http-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.82
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-http-release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-http-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.83
      }
    },
    {
      "name": "⚖️browser-test📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:browser-test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️contract-check🎮️native-host🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/playground-native-host:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️source-check📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️test📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️wasm📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-codec-oracle🌍️gis🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-plugin:native-codec-oracle",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-codec-oracle🌿️vcs🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/vcs-plugin:native-codec-oracle",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️source-check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-build💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-build-release💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-build-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-presentation🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-presentation-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2634
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-mcp🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-mcp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2635
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-native-service🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-native-service",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2636
      }
    },
    {
      "name": "⚖️check🧩️mcp-installed-service🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:installed-service-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2637
      }
    },
    {
      "name": "⚖️publish🗄️stdio-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-publication:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️test🗄️stdio-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-publication:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️build🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️check🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️test🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️canonical-architecture🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️deletion-proof🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:deletion-proof",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️flow-add-widget-retained-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:add-widget-retained-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-child-edit-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:child-edit-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-child-identity-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:child-identity-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-test-source🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:test-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-architect-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-architect-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-cad-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cad-cad-rs:verify-cad-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-curation-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/sourcing-curation-rs:verify-curation-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-dag-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dag-dag-rs:verify-dag-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-drawing-canvas-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:verify-drawing-canvas-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-assembly-physical-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-assembly-physical-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-live-visual-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-live-visual-publication",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-mesh-preparation-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-mesh-preparation-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-numerical-microcursor🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-numerical-microcursor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-numerical-page-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-numerical-page-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-pcg-publication-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-pcg-publication-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-scalar-owners-native🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-scalar-owners-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem2d-window-config-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem2d-window-config-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem3d-numerical-child-native🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/fem-3d-rs:verify-fem3d-numerical-child-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem3d-window-config-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem3d-window-config-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-flow-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:verify-flow-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-forms-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/forms-forms-rs:verify-forms-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-forms-try-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-forms-composition-tests:verify-forms-try-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-framework-flow-physical-retirement🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-flow-flow-rs:verify-framework-flow-physical-retirement",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-framework-ui-protocol-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-space-composition-tests:verify-framework-ui-protocol-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation2d-window-camera-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation2d-rs:verify-generation2d-window-camera-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation3d-document-io🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-document-io",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation3d-preview-window-transient🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-preview-window-transient",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-gis-map-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-gis-map-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-gis-terrain-window-config🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gisterrain-rs:verify-gis-terrain-window-config",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-home-host-panel-owner🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/space-home-rs:verify-home-host-panel-owner",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-jack-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-rs:verify-jack-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-jack-query-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-rs:verify-jack-query-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-layout-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/layout-layout-rs:verify-layout-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-layout-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/layout-layout-rs:verify-layout-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-map-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-map-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-norm-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-norm-composition-tests:verify-norm-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-norm-results-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-norm-composition-tests:verify-norm-results-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-note-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-note-composition-tests:verify-note-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-note-empty-config-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-note-empty-config-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-playbook-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/playbook-playbook-rs:verify-playbook-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-presentation-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/animate-presentation-rs:verify-presentation-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-procedure-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/imperative-procedure-rs:verify-procedure-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-program-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-program-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-puzzle-fill-policy-self-tests🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-puzzle-composition-tests:verify-puzzle-fill-policy-self-tests",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-remodel-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:verify-remodel-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-map-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-map-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-window-config🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-window-config",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-sequence-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/sequence-sequence-rs:verify-sequence-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-stdio-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:verify-stdio-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-terrain-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gisterrain-rs:verify-terrain-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-wires-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/reasoning-wires-rs:verify-wires-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-wires-window-transient🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/reasoning-wires-rs:verify-wires-window-transient",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-writer-window-state🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/writer-writer-rs:verify-writer-window-state",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️owned-script-routes🦑️repo🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-owned-script-routes",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️installed-service🧰️os🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:installed-service-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2631
      }
    },
    {
      "name": "⚖️inference🗺️gismap🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2632
      }
    },
    {
      "name": "⚖️inference-browser🗺️gismap🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:cold-document-pair-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2633
      }
    },
    {
      "name": "⚖️browser-dock-widgets⚛️react🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run react \"${input:wgpuDockReactServe}\" --suite widgets --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2641
      }
    },
    {
      "name": "⚖️browser-dock-widgets🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run wgpu \"${input:wgpuDockServe}\" --suite widgets --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2642
      }
    },
    {
      "name": "⚖️browser-embedded-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-embedded-acceptance -- --configuration \"${workspaceFolder}/${input:wgpuEmbeddedConfiguration}\" --output \"${workspaceFolder}/${input:wgpuEmbeddedArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2643
      }
    },
    {
      "name": "🧪️test🖍️draw🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.4
      }
    },
    {
      "name": "🧪️test🖍️draw🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.5
      }
    },
    {
      "name": "🔍️check🖍️draw🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.6
      }
    },
    {
      "name": "📚️deps print tex",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:deps-tex",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -36
      }
    },
    {
      "name": "📥️deps print compiler",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:deps-tectonic",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -37
      }
    },
    {
      "name": "🔄️watch logo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:logo-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 19
      }
    },
    {
      "name": "▶️repo coordinator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 20
      }
    },
    {
      "name": "🚀️start repo coordinator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 21
      }
    },
    {
      "name": "📥️styling Python dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-py:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "📥️styling .NET dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-dotnet:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -44
      }
    },
    {
      "name": "📥️energy oracle Python dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/energy-oracle-py:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "⚙️setup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -50
      }
    },
    {
      "name": "▶️start",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.9
      }
    },
    {
      "name": "🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.8
      }
    },
    {
      "name": "🧹lint",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:lint",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.7
      }
    },
    {
      "name": "🎨format",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:format",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.6
      }
    },
    {
      "name": "🧪️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.5
      }
    },
    {
      "name": "📦️build",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.4
      }
    },
    {
      "name": "🚢️publish",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.3
      }
    },
    {
      "name": "🧰️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49
      }
    },
    {
      "name": "⚙️setup🪟️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -48
      }
    },
    {
      "name": "⚙️setup🐙️git",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup-git",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -47
      }
    },
    {
      "name": "📥️deps javascript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-javascript",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -46
      }
    },
    {
      "name": "📥️deps python",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-python",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "📥️deps cargo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cargo",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -44
      }
    },
    {
      "name": "📥️deps go",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-go",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43
      }
    },
    {
      "name": "📥️deps dotnet",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-dotnet",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -42
      }
    },
    {
      "name": "📥️deps cpp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cpp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -41
      }
    },
    {
      "name": "📥️deps browsers",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-browsers",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -40
      }
    },
    {
      "name": "📥️deps wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -39
      }
    },
    {
      "name": "📥️deps Trunk",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-trunk",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38.75
      }
    },
    {
      "name": "📥️deps wasm optimizer",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-wasm-opt",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38.5
      }
    },
    {
      "name": "📥️deps tools",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-tools",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38
      }
    },
    {
      "name": "🧪️test⚡️cache-command-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cache-command-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": -11.5
      }
    },
    {
      "name": "⚖️gate🦀️cargo🧾️build-dir-provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:cargo-provenance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": -11.4
      }
    },
    {
      "name": "🚦️ci baseline",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:ci-baseline",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -39
      }
    },
    {
      "name": "🦑️mcp dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun ./📜️script.ts dev mcp stdio client",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -4
      }
    },
    {
      "name": "⌨️gemini",
      "type": "node-terminal",
      "request": "launch",
      "command": "gemini --yolo",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "1_keyboard",
        "order": 10
      }
    },
    {
      "name": "⌨️kiro",
      "type": "node-terminal",
      "request": "launch",
      "command": "kiro-cli chat --trust-all-tools",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "1_keyboard",
        "order": 20
      }
    },
    {
      "name": "🖱️f3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "f3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 10
      }
    },
    {
      "name": "🖱️gitkraken",
      "type": "node-terminal",
      "request": "launch",
      "command": "gitkraken --path \"${workspaceFolder}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 20
      }
    },
    {
      "name": "🖱️mcpinspector",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp",
      "env": {
        "CLIENT_PORT": "6274",
        "SERVER_PORT": "6277"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 30
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://[^\\s]+:6274)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🎛️dashboard",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌀daemon▶️start",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:daemon -- start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.1
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌀daemon📎attach",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:daemon -- attach",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.2
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌊️workflow",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:workflow",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.3
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌳️command-tree",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts run command-tree --dump-tree",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.4
      }
    },
    "@generated:cad:react",
    "@generated:cad:wgpu",
    {
      "name": "🛠️dev📐️cad🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- cad",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 10.2
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- cad fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "CAD_JS_RENDERER_PLAY_PORT": "6020",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 20
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6020)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- cad fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "CAD_JS_RENDERER_PLAY_PORT": "6120",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 20.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6120)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- cad",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 20.2
      }
    },
    "@generated:dag:react",
    "@generated:dag:wgpu",
    {
      "name": "🛠️dev🌳️dag🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- dag",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 140.2
      }
    },
    "@generated:mathematical:react",
    "@generated:mathematical:wgpu",
    {
      "name": "🛠️dev🧮️mathematical🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- mathematical",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 141.2
      }
    },
    "@generated:architect:react",
    "@generated:architect:wgpu",
    {
      "name": "🛠️dev🏛️architect🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- architect",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 142.2
      }
    },
    "@generated:flow:react",
    "@generated:flow:wgpu",
    {
      "name": "🛠️dev🌊️flow🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- flow",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 150.2
      }
    },
    "@generated:imperative:react",
    "@generated:imperative:wgpu",
    {
      "name": "🛠️dev⚙️imperative🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- imperative",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 155.2
      }
    },
    "@generated:sequence:react",
    "@generated:sequence:wgpu",
    {
      "name": "🛠️dev📜️sequence🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- sequence",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 156.2
      }
    },
    "@generated:lowpoly:react",
    "@generated:lowpoly:wgpu",
    {
      "name": "🛠️dev🔷️lowpoly🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- lowpoly",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 157.2
      }
    },
    "@generated:layout:react",
    "@generated:layout:wgpu",
    {
      "name": "🛠️dev📄️layout🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- layout",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 158.2
      }
    },
    "@generated:gis2d:react",
    "@generated:gis2d:wgpu",
    {
      "name": "🛠️dev🌐️gis📍️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- gis2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 160.2
      }
    },
    "@generated:gis3d:react",
    "@generated:gis3d:wgpu",
    {
      "name": "🛠️dev🌐️gis⛰️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- gis3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 160.5
      }
    },
    "@generated:animate:react",
    "@generated:animate:wgpu",
    {
      "name": "🛠️dev🎬️animateplay🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- animate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 170.2
      }
    },
    {
      "name": "🛠️dev🖨️print📊️viz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:watch-viz",
      "cwd": "${workspaceFolder}"
    },
    {
      "name": "🛠️dev🖨️print",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:watch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 175
      }
    },
    "@generated:generation2d:react",
    "@generated:generation2d:wgpu",
    {
      "name": "🛠️dev🔧️procedural🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- generation2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 180.2
      }
    },
    "@generated:generation3d:react",
    "@generated:generation3d:wgpu",
    {
      "name": "🛠️dev🔧️procedural🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- generation3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 190.2
      }
    },
    "@generated:process3d:react",
    "@generated:process3d:wgpu",
    {
      "name": "🛠️dev🪚️process🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- process3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 195.2
      }
    },
    "@generated:sourcing:react",
    "@generated:sourcing:wgpu",
    {
      "name": "🛠️dev🛒️sourcing🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- sourcing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 196.2
      }
    },
    "@generated:bitmap:react",
    "@generated:bitmap:wgpu",
    {
      "name": "🛠️dev🀄️wfc🖼️bitmap🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- bitmap",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 201.2
      }
    },
    "@generated:grid2d:react",
    "@generated:grid2d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🔲️grid2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- grid2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 202.2
      }
    },
    "@generated:wfc2d:react",
    "@generated:wfc2d:wgpu",
    {
      "name": "🛠️dev🀄️wfc◻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- wfc2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 203.2
      }
    },
    "@generated:grid3d:react",
    "@generated:grid3d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🧱️grid3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- grid3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 204.2
      }
    },
    "@generated:wfc3d:react",
    "@generated:wfc3d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🧊️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- wfc3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 205.2
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6018",
        "SEMIO_RENDERER": "react",
        "SEMIO_DEFAULT_EXAMPLE": "hexagonal-mushroom-column"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6018)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6118",
        "SEMIO_RENDERER": "wgpu",
        "SEMIO_DEFAULT_EXAMPLE": "hexagonal-mushroom-column"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6118)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d👁️viewer⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6018",
        "SEMIO_RENDERER": "react",
        "SEMIO_APP_ROLE": "viewer"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.2
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6018)",
        "uriFormat": "%s/?plugin=generation3d"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d👁️viewer🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6118",
        "SEMIO_RENDERER": "wgpu",
        "SEMIO_APP_ROLE": "viewer"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.3
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6118)",
        "uriFormat": "%s/?plugin=generation3d&role=viewer"
      }
    },
    {
      "name": "🛠️dev📽️projektetage",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-praesentation-projektetage:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "PRAESENTATION_PROJEKTETAGE_PORT": "6050"
      },
      "presentation": {
        "group": "3_dev",
        "order": 210
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6050)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📚️bericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 211
      }
    },
    "@generated:aggregator:react",
    {
      "name": "🛠️dev♻️mit-bestand🧺️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "MIT_BESTAND_DEMONSTRATOR_PORT": "6029",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6029)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "♻️activate🏚️mitbestand🎪️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.1
      }
    },
    {
      "name": "🖥️serve🏚️mitbestand🎪️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:serve",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.2
      }
    },
    {
      "name": "🛠️dev🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TECH_PLAY_PORT": "6033",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.3
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6033)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "♻️activate🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:activate-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.4
      }
    },
    {
      "name": "🖥️serve🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:serve",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.5
      }
    },
    {
      "name": "🛠️dev🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "TEACHING_ARCHITECTURE_QUIZ_PORT": "6061",
        "PROCTOR_PORT": "8791"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.6
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6061)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "PROCTOR_PORT": "8791"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.61
      }
    },
    {
      "name": "🔁️rebuild🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:rebuild",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.611
      }
    },
    {
      "name": "🧪️test❓️quiz🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.63
      }
    },
    {
      "name": "🧪️test❓️quiz⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-react:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.64
      }
    },
    {
      "name": "🧪️test❓️quiz🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.65
      }
    },
    {
      "name": "🧪️test🎓️teaching🛂️proctor🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.66
      }
    },
    {
      "name": "🧪️test🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.67
      }
    },
    {
      "name": "🛠️dev❓️quiz⚛️react🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-react:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.68
      }
    },
    "@generated:generator:react",
    "@generated:koordinator:react",
    "@generated:aussuchen:react",
    "@generated:bearbeiten:react",
    "@generated:verfolgen:react",
    "@generated:puzzle2d:react",
    "@generated:puzzle2d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 220.2
      }
    },
    "@generated:puzzle3d:react",
    "@generated:puzzle3d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 230.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 3d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_3D_PLAY_PORT": "6013",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 240
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6013)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 3d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_3D_PLAY_PORT": "6113",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 240.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6113)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 240.2
      }
    },
    "@generated:puzzle5d:react",
    "@generated:puzzle5d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle👯️5d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 250.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6014",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 260
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6014)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6114",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 260.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6114)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 260.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️capsule🌙️dream⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6015",
        "SEMIO_RENDERER": "react",
        "PLAYGROUND_LOCKED_EXAMPLE_ID": "capsule-dream",
        "SEMIO_DEFAULT_EXAMPLE": "capsule-dream"
      },
      "presentation": {
        "group": "3_dev",
        "order": 261
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6015)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️capsule🌙️dream🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6115",
        "SEMIO_RENDERER": "wgpu",
        "PLAYGROUND_LOCKED_EXAMPLE_ID": "capsule-dream",
        "SEMIO_DEFAULT_EXAMPLE": "capsule-dream"
      },
      "presentation": {
        "group": "3_dev",
        "order": 261.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6115)",
        "uriFormat": "%s"
      }
    },
    "@generated:block2d:react",
    "@generated:block2d:wgpu",
    {
      "name": "🛠️dev🧱️block🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 261.2
      }
    },
    "@generated:block3d:react",
    "@generated:block3d:wgpu",
    {
      "name": "🛠️dev🧱️block🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 262.2
      }
    },
    "@generated:block5d:react",
    "@generated:block5d:wgpu",
    {
      "name": "🛠️dev🧱️block👯️5d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 263.2
      }
    },
    "@generated:reasoning-wires:react",
    "@generated:reasoning-wires:wgpu",
    {
      "name": "🛠️dev🧠️reasoning🔗️wires🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- reasoning-wires",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 270.2
      }
    },
    {
      "name": "🛠️dev🧰️repo🤖️mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp repo",
      "env": {
        "MCP_AUTO_OPEN_ENABLED": "false",
        "MCP_PROXY_AUTH_TOKEN": "repo-mcp-token"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://[^\\s]+:6274/\\?MCP_PROXY_AUTH_TOKEN=[^\\s]+)",
        "uriFormat": "%s&transport=stdio&serverCommand=cargo&serverArgs=run&serverArgs=--release&serverArgs=-p&serverArgs=semio-framework-repo-cli&serverArgs=--&serverArgs=mcp&MCP_PROXY_FULL_ADDRESS=http://127.0.0.1:6277"
      }
    },
    {
      "name": "🛠️dev🧰️repo🤖️mcp⌨️cursor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp stdio cursor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.1
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️cli🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run --args=\"--help\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.15
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️cli🦀️semio-repo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:repo --args=\"--help\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.16
      }
    },
    {
      "name": "🛠️dev🧰️repo🔌️mcp🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:mcp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.17
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️client",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run",
      "env": {
        "SEMIO_REPO_IMPLEMENTATION": "go",
        "GOWORK": "${workspaceFolder}/go.work"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.25
      }
    },
    {
      "name": "🛠️build🧰️repo🔌️mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo-mcp:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.4
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🐹️go",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator-go:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.5
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator-rs:run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.6
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🟦️typescript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.7
      }
    },
    "@generated:shooting:react",
    "@generated:shooting:wgpu",
    {
      "name": "🛠️dev📸️shooting🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- shooting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 290.2
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- shooting fixture base-icon",
      "cwd": "${workspaceFolder}",
      "env": {
        "SHOOTING_PLAY_PORT": "6019",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 300
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6019)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- shooting fixture base-icon",
      "cwd": "${workspaceFolder}",
      "env": {
        "SHOOTING_PLAY_PORT": "6119",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 300.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6119)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- shooting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 300.2
      }
    },
    {
      "name": "🛠️dev📖️storybook",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 310
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle◻️2d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-2d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 340
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🖱️ui",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-ui",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 350
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🎨️styling",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-styling",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 460
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle🧊️3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 470
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle🖐️5d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 480
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 490
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework🔌️hosts",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework-hosts",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 400
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework🖥️os",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework-os",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 410
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook♾️infinite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-infinite",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 420
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook📐️cad",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-cad",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 430
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🎬️animate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-animate",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 450
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    "@generated:trinity-jack:react",
    "@generated:trinity-jack:wgpu",
    {
      "name": "🛠️dev🔺️trinity🃏️jack🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- trinity",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 360.2
      }
    },
    {
      "name": "🛠️dev🔺️trinity🃏️jack🦀️shell",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-shell:run -- trinity/fixture/nakagin-capsule-tower.trinity.json \"MATCH (a:Piece) RETURN a.name\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 370
      }
    },
    "@generated:trinity-rewriting:react",
    "@generated:trinity-rewriting:wgpu",
    {
      "name": "🛠️dev🔺️trinity♻️rewriting🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- trinity-rewriting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 380.2
      }
    },
    "@generated:forms:react",
    "@generated:forms:wgpu",
    {
      "name": "🛠️dev📋️forms🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- forms",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 385.2
      }
    },
    {
      "name": "🧪️test🖨️raster🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/raster-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.7
      }
    },
    {
      "name": "🧪️test🖨️raster🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/raster-raster-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.8
      }
    },
    {
      "name": "🧪️test🔲️pixels🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/pixels:test-typescript",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.9
      }
    },
    {
      "name": "🧪️test🔲️pixels🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/pixels:test-rust",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387
      }
    },
    "@generated:raster:react",
    "@generated:raster:wgpu",
    {
      "name": "🛠️dev🖼️raster🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- raster",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.2
      }
    },
    "@generated:vcs:react",
    "@generated:vcs:wgpu",
    {
      "name": "🛠️dev🗄️vcs🎛️play🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- vcs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 385.2
      }
    },
    "@generated:s:react",
    "@generated:s:wgpu",
    "@generated:s:users",
    {
      "name": "🛠️dev🖥️s⚛️react📦️served",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- s served",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6070",
        "SEMIO_PLUGIN": "s",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 386.05
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6070)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🖥️s🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.1
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:plugin",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.5
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins📏️size",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:plugin -- size",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.6
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins🧫️scale-fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:generate-scale-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.7
      }
    },
    {
      "name": "🛠️dev🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-postgres",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev-postgres/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.002
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-neo4j",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev-neo4j/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.003
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends⬆️up",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-up -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.004
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends🩺️status",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-status -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.005
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends⬇️down",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-down -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.006
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🎫️local",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-hub-owner",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_HUB_URL": "http://127.0.0.1:8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.01
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🛡️admin",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-admin:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_URL": "http://127.0.0.1:8787"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.05
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8790)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🪐️space⚛️react🔒local-only",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- s",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6070",
        "SEMIO_PLUGIN": "s",
        "SEMIO_RENDERER": "react",
        "S_LOCAL_ONLY": "1"
      },
      "presentation": {
        "group": "3_dev",
        "order": 386.25
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6070)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-suite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-suite",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "S_OS_PORT": "6066",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.06
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-native🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/",
        "SEMIO_PLUGIN": "s"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.061
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-mcp🌉️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-mcp",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.062
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-admin🛡️browser",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-admin",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.063
      }
    },
    {
      "name": "🛠️dev🤝️os-collab-e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-collaboration",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.064
      }
    },
    {
      "name": "🛠️dev🪐️os-s🩺️cold-boot",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:cold-boot-check-s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.065
      }
    },
    {
      "name": "🛠️dev🪐️os-s🔭️foreign-kind",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:s-host-foreign-kind-s -- http://127.0.0.1:6070/ --tag launch raster dag block=s.block.block2d@1/*#editor block=s.block.block3d@1/*#editor block=s.block.block5d@1/*#editor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.066
      }
    },
    {
      "name": "🛠️dev🪐️os-s🤏️pinch-a11y",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s -- http://127.0.0.1:6070/ --tag launch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.067
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🧵️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp stdio os",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.05
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🌐️http",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp http os",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.06
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🤝️client-e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp:client-e2e",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.07
      }
    },
    {
      "name": "🛠️dev⏳️async🛌️worker-parking-check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-async-rs:worker-parking-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.071
      }
    },
    {
      "name": "🖱️mcpinspector🌉️os",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x @modelcontextprotocol/inspector --config .mcp.json --server semio",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 31
      }
    },
    {
      "name": "🛠️dev🕸️os-run",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:os -- run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.1
      }
    },
    "@generated:draw:react",
    "@generated:draw:wgpu",
    {
      "name": "🛠️dev✏️draw🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- draw",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.2
      }
    },
    "@generated:note:react",
    "@generated:note:wgpu",
    {
      "name": "🛠️dev📝️note🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- note",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 388.2
      }
    },
    "@generated:writer:react",
    "@generated:writer:wgpu",
    {
      "name": "🛠️dev✍️writer🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- writer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.2
      }
    },
    "@generated:remodel:react",
    "@generated:remodel:wgpu",
    {
      "name": "🛠️dev🏺️remodel🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- remodel",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 389.2
      }
    },
    {
      "name": "🛠️dev🖱️ui🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-react:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390
      }
    },
    {
      "name": "🛠️dev🧰️framework🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390.1
      }
    },
    {
      "name": "🛠️dev💻️os🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390.2
      }
    },
    "@generated:fem2d:react",
    "@generated:fem2d:wgpu",
    {
      "name": "🛠️dev🏗️fem🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- fem2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 392.2
      }
    },
    "@generated:fem3d:react",
    "@generated:fem3d:wgpu",
    {
      "name": "🛠️dev🏗️fem🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- fem3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 392.5
      }
    },
    {
      "name": "🛠️dev🧊️wgpu🖥️native🚢️release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native-release -- s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 121.4
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📋️zwischenbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-zwischenbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📑️forschungsbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-forschungsbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392.001
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand kompaktbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-kompaktbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392.002
      }
    },
    {
      "name": "📦️build✏️s",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 9
      }
    },
    {
      "name": "📦️build🖥️s",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-s-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 10
      }
    },
    {
      "name": "📦️build📐️cad",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-cad-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 10
      }
    },
    {
      "name": "📦️build🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11
      }
    },
    {
      "name": "⚖️gate🏢️semio-tech🎡️play🎭️e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:test-e2e",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11
      }
    },
    {
      "name": "📦️build🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.1
      }
    },
    {
      "name": "🚚️publish🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.15
      }
    },
    {
      "name": "📦️build🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.2
      }
    },
    {
      "name": "📦️build🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.3
      }
    },
    {
      "name": "🚚️publish🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.35
      }
    },
    {
      "name": "✅️check🎓️teaching🏛️architecture❓️quiz📚️catalog",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.1
      }
    },
    {
      "name": "✅️check🎓️teaching🛂️proctor📚️catalog",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.15
      }
    },
    {
      "name": "⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.2
      }
    },
    {
      "name": "⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-stack",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-stack-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.25
      }
    },
    {
      "name": "📦️build🧩️puzzle🏙️3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-puzzle3d-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 140
      }
    },
    {
      "name": "📦️build🧩️puzzle👯️5d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-puzzle5d-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 150
      }
    },
    {
      "name": "📦️build📸️shooting",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-shooting-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 180
      }
    },
    {
      "name": "📦️check🧊️wgpu🔒️lockfile",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:lockfile-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.145
      }
    },
    {
      "name": "📦️generate🧊️wgpu🚀️boot",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:generate-browser-boot",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.146
      }
    },
    {
      "name": "📦️check🧊️wgpu🌐️browser",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.147
      }
    },
    {
      "name": "📦️check⚛️react🔐️hub-sign-in",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-react:hub-sign-in-spaces-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1475
      }
    },
    {
      "name": "📦️test🧊️wgpu🖱️ui",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-wgpu-engine",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.148
      }
    },
    {
      "name": "📦️test🧊️wgpu📺️renderer",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.149
      }
    },
    {
      "name": "📦️test🧊️wgpu♾️infinite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-infinite:test-wgpu-world-terrain",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.15
      }
    },
    {
      "name": "📦️build🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.159
      }
    },
    {
      "name": "📦️build-dev🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:build-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16
      }
    },
    {
      "name": "🚚️publish🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1601
      }
    },
    {
      "name": "📦️generate🌐️gis🧬️native-codec-projection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-plugin:native-codec-projection",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16013
      }
    },
    {
      "name": "📦️generate🗄️stdio🧬️native-codec-projection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:native-codec-projection",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16014
      }
    },
    {
      "name": "🛫️preflight-catalog🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:trusted-catalog-preflight --packages all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.160145
      }
    },
    {
      "name": "🚚️publish-catalog🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:trusted-catalog-bootstrap --packages all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16015
      }
    },
    {
      "name": "🧊️check-guest-framework🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:guest-framework-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.160155
      }
    },
    {
      "name": "🔁️rebuild-all🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:rebuild-all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16016
      }
    },
    {
      "name": "📦️build-release🌉️os-mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:build-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1602
      }
    },
    {
      "name": "🚚️publish🌉️os-mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1603
      }
    },
    {
      "name": "📦️test🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.161
      }
    },
    {
      "name": "📦️test🗄️os-hub♾️all-features",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:test-all-features",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.162
      }
    },
    {
      "name": "📦️test🖥️server",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-server-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1625
      }
    },
    {
      "name": "📦️test🖥️server🟦️typescript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-server:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1626
      }
    },
    {
      "name": "📦️check🗄️os-hub🚀️launch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-bootstrap-launch-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.163
      }
    },
    {
      "name": "📦️check🤖️generated🔬️corruption",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-generated-corruption",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.164
      }
    },
    {
      "name": "📦️check🗄️os-hub🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.165
      }
    },
    {
      "name": "📦️check🦑️repo🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.166
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-clean-scaffold-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-clean-scaffold-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.167
      }
    },
    {
      "name": "🧬schema🗺️surface🏛️abstraction🧪source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-surface-abstraction-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.168
      }
    },
    {
      "name": "🧬schema🗿artifact🧪root-artifact-schema-law-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-artifact-schema-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.169
      }
    },
    {
      "name": "🧬schema💡️inference🧪source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-inference-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.17
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-schema-field-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-schema-field-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.171
      }
    },
    {
      "name": "🧹clean🦑️repo🧪️source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-repo-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.172
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎚️vitest-configuration-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-vitest-configuration-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.173
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎚️tool-configuration-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-tool-configuration-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.174
      }
    },
    {
      "name": "🧹clean🧩️taxonomy📦️package-body-policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-package-body-policy",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.175
      }
    },
    {
      "name": "📦️test🥾️bootstrap🪟️cross-platform",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cross-platform-bootstrap",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.176
      }
    },
    {
      "name": "📦️test🏃️process🪓️tree-termination",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-process-tree-termination",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.177
      }
    },
    {
      "name": "📦️test🪟️windows🧭️command-paths",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-windows-command-paths",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1775
      }
    },
    {
      "name": "📦️test🦀️cargo🧾️provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cargo-provenance",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.178
      }
    },
    {
      "name": "🧹clean🦀️cargo🧾️provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:cargo-provenance-repair",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.179
      }
    },
    {
      "name": "📦️check🗿️taxonomy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-taxonomy-report",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.165
      }
    },
    {
      "name": "📦️check🗿️taxonomy🧩️implementation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-taxonomy-implementation-report",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.166
      }
    },
    {
      "name": "📦️check🔒️dependencies📃️literal-external",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-dependencies-literal-external",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.167
      }
    },
    {
      "name": "🏛️check🧩️canonical-architecture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 899.99
      }
    },
    {
      "name": "📦️check🧅️layering",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-layering",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.168
      }
    },
    {
      "name": "📦️check🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.169
      }
    },
    {
      "name": "📦️generate🖨️print📊️viz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:generate-viz",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1689
      }
    },
    {
      "name": "📦️generate📕️norm🪨️en1996🖼️example-assets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:regenerate-example-assets",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16915
      }
    },
    {
      "name": "📦️generate🧬️surface-schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:surface-schema",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1691
      }
    },
    {
      "name": "📦️check🧬️surface-schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:surface-schema-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1692
      }
    },
    {
      "name": "📦️generate✨️dsl-derive🧬️mutation-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dsl-derive-rs:generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1693
      }
    },
    {
      "name": "📦️check✨️dsl-derive🧬️mutation-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dsl-derive-rs:check-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1694
      }
    },
    {
      "name": "📦️check🧹️fixture-sweep",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fixture-sweep-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.17
      }
    },
    {
      "name": "📦️check🌊️flow-composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-flow-composition-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1701
      }
    },
    {
      "name": "📦️check🌐️semio-session",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-spatial-kernel-semio-session-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1702
      }
    },
    {
      "name": "📦️check📡️channel-version",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.171
      }
    },
    {
      "name": "🛠️dev📡️channel-version🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.172
      }
    },
    {
      "name": "📦️wasm🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 500.81
      }
    },
    {
      "name": "📦️wasm-release🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:wasm-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 500.811
      }
    },
    {
      "name": "⚖️gate🧱️hub-foundations📐️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:foundation-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10755
      }
    },
    {
      "name": "⚖️gate🧭️local-relay📛️admission",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-relay-routing-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10756
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🤝️live-sign-in",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:live-sign-in-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10757
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-journey",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-journey-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-creation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-creation-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-collaboration",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-collaboration-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🧊️wgpu⏯️native-guest-journey",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native-guest-journey-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚨️capability-audit",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:capability-audit-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤖️live-agent-loop",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:live-agent-loop-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10758
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤖️live-agent-loop🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:live-agent-loop-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_MCP_LIVE_LOCALE": "de"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.107582
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤝️hub-edit-durability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:hub-edit-durability-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107585
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤝️hub-agent-participant",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:hub-agent-participant-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10759
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp💬️agent-reply",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:agent-reply-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107595
      }
    },
    {
      "name": "🧹clean🧩️taxonomy❄️frozen-markdown-coordinates",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-frozen-markdown-coordinates --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.18
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🕰️historical-json-source-encoding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-historical-json-source-encoding --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.19
      }
    },
    {
      "name": "⚖️gate🌊️flow🌐️startup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.8
      }
    },
    {
      "name": "⚖️gate🌊️flow⏱️consumed-clock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser-clock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.81
      }
    },
    {
      "name": "⚖️gate🌊️flow🏷️browser-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.82
      }
    },
    {
      "name": "📦️preview🤖️flow-browser-package",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:preview-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.83
      }
    },
    {
      "name": "📦️build🗄️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.84
      }
    },
    {
      "name": "📦️check🗄️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.85
      }
    },
    {
      "name": "⚖️gate🗄️stdio🧩️composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.86
      }
    },
    {
      "name": "⚖️gate🗄️stdio🛂️package-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:package-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.87
      }
    },
    {
      "name": "⚖️gate🗄️stdio🕸️package-graph",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:package-graph",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.88
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editing🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-artifact-contract-rs:test -- --lib",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.89
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editors🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --target=test --projects=\"@semio-tech/stdio-*-rs\" --exclude=@semio-tech/stdio-artifact-contract-rs --parallel=2 -- --features component-app-assembly --lib editor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.9
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️catalog🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --test editor_catalog",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.91
      }
    },
    {
      "name": "⚖️gate🗄️stdio🚢️shipped-fleet🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --test shipped_fleet",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.915
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editing🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-snapshot-editing-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.92
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎬️lanes🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-scene-rs:test -- --lib lanes_preserve_complete_unicode_documents",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.93
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎬️lanes🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-scene-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.94
      }
    },
    {
      "name": "⚖️gate🗄️stdio🪟️kits🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:test -- window_kits_tests",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.95
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️component🌐️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:editor-component-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.96
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️fixture🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- editor-catalog-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.97
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️launch🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test -- 🧪️tests/🚀️launch/🟦️.ts",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.98
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.32
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.33
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident-rs:check-wasm --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.34
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🔣️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:test-source --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.35
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.36
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🦀️check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.37
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:check-wasm --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.38
      }
    },
    {
      "name": "⚖️gate🔌️plugin🦀️lib",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.39
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-support",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-support --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.06
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🏺️historical-package-owner-identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-historical-package-owner-identity --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.07
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧲️rust-physical-reference-context",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-physical-reference-context --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.08
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️cli-cancellation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-cli-cancellation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.09
      }
    },
    {
      "name": "🧹clean🧩️taxonomy💠️inventory-artifact-shards",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-inventory-artifact-shards --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.1
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-script-compiler",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-script-compiler",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.11
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔎️json-reference-owner-lookup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-json-reference-owner-lookup --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.12
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🚧️cargo-discovery-exclusions",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-discovery-exclusions --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.13
      }
    },
    {
      "name": "🧹clean🧩️taxonomy💥️nested-cargo-collision-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nested-cargo-collision-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.14
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🌐️registry-import-language",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-registry-import-language --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.15
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🛫️preflight-reference-basis",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-preflight-reference-basis --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.16
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🛤️typescript-path-collection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-typescript-path-collection --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.17
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🗺️testing-readme-coordinates",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-testing-readme-coordinates --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.192
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-source-residue",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-source-residue --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.194
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔖️readme-current-source-revision",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-current-source-revision --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.195
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🚚️readme-move-source-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-move-source-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.197
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-source-commit",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-source-commit --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.198
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🫙️artifact-empty-facet-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-empty-facet-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.199
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🟢️readme-current-source-activation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-current-source-activation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.204
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🪶️artifact-empty-facet-authoring",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-empty-facet-authoring --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.205
      }
    },
    {
      "name": "🧹clean🧩️taxonomy👀️readme-reviewed-fixture-inputs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-reviewed-fixture-inputs --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.206
      }
    },
    {
      "name": "🧹clean🧩️taxonomy♻️taxonomy-pattern-compiler-reuse",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-pattern-compiler-reuse --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.207
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔤️taxonomy-leading-grapheme",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-leading-grapheme --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.209
      }
    },
    {
      "name": "🧹clean🧩️taxonomy📈️reference-coordinate-progress",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-reference-coordinate-progress --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.21
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎯️draw-destination-observation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-draw-destination-observation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.211
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧾️registry-catalog-gitlink-boundary",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-registry-catalog-gitlink-boundary --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.2152
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎯️cargo-target-discovery-skip",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-target-discovery-skip --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.2154
      }
    },
    {
      "name": "⚖️gate🔌️socket-grant📐️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:socket-grant-command-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.09999
      }
    },
    {
      "name": "⚖️gate🔌️socket-grant🛡️server",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:socket-grant-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.1
      }
    },
    {
      "name": "⚖️gate📌️check-in🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.11
      }
    },
    {
      "name": "⚖️gate📌️check-in🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.12
      }
    },
    {
      "name": "⚖️gate📌️check-in🌉️process",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-process-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.13
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.15
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.151
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.152
      }
    },
    {
      "name": "⚖️gate🗂️directory-live-lanes🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run os-hub:directory-live-lanes -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.153
      }
    },
    {
      "name": "⚖️gate🗂️directory-live-lanes🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run os-hub:directory-live-lanes -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.154
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.16
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.17
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.18
      }
    },
    {
      "name": "⚖️gate📈️document-growth🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.19
      }
    },
    {
      "name": "⚖️gate📈️document-growth🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2
      }
    },
    {
      "name": "⚖️gate📈️document-growth🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.21
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-goal",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.22
      }
    },
    {
      "name": "⚖️browser-dock-contract🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.26
      }
    },
    {
      "name": "⚖️browser-dock-acceptance⚛️react🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run react \"${input:wgpuDockReactServe}\" --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.261
      }
    },
    {
      "name": "⚖️browser-dock-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run wgpu \"${input:wgpuDockServe}\" --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.262
      }
    },
    {
      "name": "⚖️browser-media-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-media-acceptance -- --serve \"${input:wgpuMediaServe}\" --locale ${input:wgpuMediaLocale} --output \"${workspaceFolder}/${input:wgpuMediaArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.263
      }
    },
    {
      "name": "⚖️parity-journey🧑‍💻dev🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:parity-journey -- --renderer ${input:rendererParityRenderer} --react-serve \"${input:wgpuDockReactServe}\" --wgpu-serve \"${input:wgpuDockServe}\" --locale ${input:wgpuMediaLocale} --output \"${workspaceFolder}/${input:rendererParityJourneyArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.264
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal🔗️hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-goal -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --local-serve ${input:acceptanceLocalServeUrl} --users ${input:acceptanceUsers}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.221
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal📋️plan",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-plan",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.23
      }
    },
    {
      "name": "⚖️gate🧮️program-matrix⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:program-matrix -- --serve http://127.0.0.1:6070/ --tag launch-en --locale en --roles editor,viewer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.24
      }
    },
    {
      "name": "⚖️gate🧮️program-matrix⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:program-matrix -- --serve http://127.0.0.1:6070/ --tag launch-de --locale de --roles editor,viewer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.25
      }
    },
    {
      "name": "⚖️gate⏯️tool-run⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:tool-run-matrix -- --serve http://127.0.0.1:6070/ --tag launch-en --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.251
      }
    },
    {
      "name": "⚖️gate⏯️tool-run⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:tool-run-matrix -- --serve http://127.0.0.1:6070/ --tag launch-de --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.252
      }
    },
    {
      "name": "⚖️gate🗂️hub-document-sweep⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:hub-document-sweep -- --serve http://127.0.0.1:6071/ --hub ${input:acceptanceHubUrl} --tag launch-en --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.253
      }
    },
    {
      "name": "⚖️gate🗂️hub-document-sweep⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:hub-document-sweep -- --serve http://127.0.0.1:6071/ --hub ${input:acceptanceHubUrl} --tag launch-de --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.254
      }
    },
    {
      "name": "⚖️gate🚪️io-matrix⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:io-matrix -- --serve http://127.0.0.1:6070/ --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.255
      }
    },
    {
      "name": "⚖️gate🚪️io-matrix⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:io-matrix -- --serve http://127.0.0.1:6070/ --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.256
      }
    },
    {
      "name": "⚖️gate👥️two-human⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:two-human -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --users ${input:acceptanceUsers} --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.26
      }
    },
    {
      "name": "⚖️gate👥️two-human⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:two-human -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --users ${input:acceptanceUsers} --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.27
      }
    },
    {
      "name": "⚖️gate🔀️connection-budget⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:connection-budget -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.275
      }
    },
    {
      "name": "⚖️gate💤️idle-budget⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:idle-budget -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.276
      }
    },
    {
      "name": "⚖️gate🫧️memory-soak⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:memory-soak -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.277
      }
    },
    {
      "name": "⚖️gate⏱️interaction-latency⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:interaction-latency -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.278
      }
    },
    {
      "name": "⚖️gate⏪️time-travel⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6012/ --renderer react --locales en,de --chords en,de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2781
      }
    },
    {
      "name": "⚖️gate⏪️time-travel🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6112/ --renderer wgpu --locales en,de --chords en,de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2782
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🧩️plugin-coverage",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:plugin-coverage-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.28
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚶️user-path",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:user-path-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "S_OS_MCP_LIVE_SHELL_URL": "${input:acceptanceServeUrl}",
        "S_OS_MCP_LIVE_LOCALE": "en"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.29
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚶️user-path🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:user-path-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "S_OS_MCP_LIVE_SHELL_URL": "${input:acceptanceServeUrl}",
        "S_OS_MCP_LIVE_LOCALE": "de"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🛡️security",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:security-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "OS_HUB_ADMIN_CAPABILITY_FILE": "${input:acceptanceHubAdminCapability}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.301
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp💼️inference-quartet",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:inference-quartet-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.302
      }
    },
    {
      "name": "⚖️gate💾️hub-backup-restore",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backup-restore-drill",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️gate🛑️hub-graceful-shutdown",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:shutdown-drill",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.315
      }
    },
    {
      "name": "⚖️gate🧠️hub-residency",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:residency-watch -- --hub ${input:acceptanceHubUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️gate🌅️hub-boot-watch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:boot-watch -- --restarts 1",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.325
      }
    },
    {
      "name": "⚖️gate🏷️hub-freshness",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:hub-freshness -- --hub ${input:acceptanceHubUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️gate🤖️hub-agent-ceiling",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:agent-ceiling-check -- --hub ${input:acceptanceHubUrl} --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.335
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.331
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.332
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.333
      }
    },
    {
      "name": "⚖️gate🚧️production-placeholders",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- production-placeholders",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.34
      }
    },
    {
      "name": "⚖️gate🎛️command-reachability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity commands",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.35
      }
    },
    {
      "name": "⚖️gate🪆️composed-child-refs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- composed-child-refs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.42
      }
    },
    {
      "name": "⚖️gate🚫️history-closure",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- history-closure",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.425
      }
    },
    {
      "name": "⚖️gate🎯️mutation-outcome-law🧪️planted",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:test-outcome-law-gate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.427
      }
    },
    {
      "name": "⚖️gate⚡️interactivity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.43
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🎯️tool-jobs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity tool-jobs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.44
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🧭️apps",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity apps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.45
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🧭️apps🎛️actions",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity apps --actions",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.46
      }
    },
    {
      "name": "⚖️gate📦️dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- dependencies",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.47
      }
    },
    {
      "name": "⚖️gate📦️dependencies0️⃣",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- dependencies literal-external",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.48
      }
    },
    {
      "name": "⚖️gate🧿️semio✉️base🔬️carrier-reproduce",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/probe-stdio-semio-v1-base:carrier-reproduce",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.49
      }
    },
    {
      "name": "⚖️gate🧪️test🏭️inventory",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:test-inventory",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.5
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-plugin-publication-source-ownership📚️library🟦️",
      "command": "bun nx run @semio-tech/repo-lib:test-plugin-publication-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.04
      }
    },
    {
      "name": "⚖️gate🔌️plugin📇️catalog🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-catalog-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05667
      }
    },
    {
      "name": "⚖️gate🔌️plugin🧾️schema-owner🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:schema-document-authority-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05668
      }
    },
    {
      "name": "⚖️gate🔁️graph revision📚️repo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-graph-revision",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1806
      }
    },
    {
      "name": "⚖️gate🪪️installation identity🧰️framework",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework:installation-identity-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1805
      }
    },
    {
      "name": "📥️deps javascript lock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-lock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43.8
      }
    },
    {
      "name": "⚖️gate📦️javascript dependency commands",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1804
      }
    },
    {
      "name": "📥️deps cargo lock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cargo-lock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43.9
      }
    },
    {
      "name": "⚖️gate🔌️plugin📦️deployment🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-deployment-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05669
      }
    },
    {
      "name": "⚖️gate🔌️plugin🚀️launch🏷️name🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-launch-name-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0567
      }
    },
    {
      "name": "🧰️framework 🏃️process 🧭️routing 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-routing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05671
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 💻️os 🔖️channel-version 🏭️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-ownership-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05672
      }
    },
    {
      "name": "🧰️framework 💻️os 🎮️playground ⭐️default 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-playground-default-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05673
      }
    },
    {
      "name": "🧰️framework 💻️os 🔖️channel version 📣️contributions 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-contributions-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05674
      }
    },
    {
      "name": "🛡️styling verification contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-verification-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05675
      }
    },
    {
      "name": "📏️styling relative sizing",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-relative-sizing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05676
      }
    },
    {
      "name": "🎥️video container providers",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:verify-video-container-providers",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05677
      }
    },
    {
      "name": "🗒️note artifact document contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-js:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05678
      }
    },
    {
      "name": "🧰️framework modules 🛍️product dependency direction",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-framework-module-product-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05679
      }
    },
    {
      "name": "🧰️framework 🦑️repo ⚡️cache owner policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cache-policy",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.056795
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🟦️workspace 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:bun-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568099999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🔍️members",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:members-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568199999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 📣️members",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:members-write",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05683
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05684
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🟦️workspace 📦️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568499999999
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 🧩️composition 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:composition-prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568599999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 📥️runtime 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:runtime-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05687
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🧩️capability 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:capability-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05688
      }
    },
    {
      "name": "✏️s 🧿️semio 🧩️composition 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:composition-prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05689
      }
    },
    {
      "name": "✏️s 🔊️wav 🧩️architecture 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-wav-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0569
      }
    },
    {
      "name": "🎭️styling color primitives",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-color-primitives",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05696
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 📼️artifact 🧪️removal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:parent-removal-check -- \"${workspaceFolder}/${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 900.05691
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 📼️artifact 🧪️tests-removal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:parent-removal-test-check -- \"${workspaceFolder}/${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 900.05692
      }
    },
    {
      "name": "🧹clean🧬️schema🧩️subset-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-subset-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05693
      }
    },
    {
      "name": "🧹clean🥽️mesh🛡️transport-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-mesh-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05694
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-taxonomy-workflow-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-taxonomy-workflow-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05695
      }
    },
    {
      "name": "🧰️framework 🏃️process 📦️artifact files 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-artifact-files",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05697
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️ Framework Process Test Budgets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-process:test-budget",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056975
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️ Framework Test Adapter Ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-test:test-adapter-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056976
      }
    },
    {
      "name": "🪪️ Framework Identity Grapheme",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-identity:test-grapheme",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056977
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:identityContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🎛️owned execution 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-owned-execution",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05698
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🪓️tree termination 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-process-tree-termination",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05699
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️repo 🎨️workspace styling 📏️pixel policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-styling-pixels",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05701
      }
    },
    {
      "name": "🦑️repo 🎨️workspace styling 🎭️color policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-styling-colors",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05702
      }
    },
    {
      "name": "🗄️stdio 🦛️Semio 🧬️conversion definition contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:conversion-definition-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/conversion-definition"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05703
      }
    },
    {
      "name": "🗄️stdio 🧊️GLTF 🧬️canonical architecture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-gltf-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05704
      }
    },
    {
      "name": "🧰️framework 🏃️process ⏱️execution budgets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-execution-budget",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05705
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🧪️bounded test command",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-test-command",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05706
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️repo 🧬️native input vocabulary",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-native-input-vocabulary",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05707
      }
    },
    {
      "name": "🗄️stdio 🧊️GLTF 🧬️native schema identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-gltf-rs:native-schema-check -- \"${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/gltf-native-schema"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05708
      }
    },
    {
      "name": "🧰️framework 🖼️assets 🗺️tile proxy 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-tile-proxy-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05709
      }
    },
    {
      "name": "🧪️framework🏃️process🦀️cargo🧭️driver",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-cargo-driver",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05711
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️framework🖼️assets🧭️dispatch🛂️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-dispatch-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05712
      }
    },
    {
      "name": "🖥️OS 🎭️actor transport 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:test-actor-transport",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0571
      }
    },
    {
      "name": "🖥️OS 🎬️media transport 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:test-media-transport",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.057101
      }
    },
    {
      "name": "🦑️Repo ⚙️native source ownership 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-native-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.057102
      }
    },
    {
      "name": "🕸️Graph 🛂️manifest 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-graph:test-manifest-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05713
      },
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:graphContractArtifacts}"
      }
    },
    {
      "name": "🖋️SVG 🎥️video 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-svg-video-contract",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:svgVideoArtifacts}"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05719
      }
    },
    {
      "name": "🏃️Process 🔒️resource leases 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-resource-leases",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0572
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🏃️Process 🧪️Vitest 🧪️driver contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-vitest-driver",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05721
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧬️Schema 🏷️entity ownership 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-entity-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05714
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:generate-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05715
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 👁️preview",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:preview-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05716
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds ✅️check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:check-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05717
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:test-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05718
      }
    },
    {
      "name": "🪧️Logo 🎬️video export",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:logo",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:svgVideoArtifacts}"
      },
      "presentation": {
        "group": "2_build",
        "order": 206.061
      }
    },
    {
      "name": "🏃️Process 📦️artifact publication 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-artifact-publication",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05722
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🏃️Process 📋️owner context 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-owner-context",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05723
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🗄️stdio 🧾️JSON 🧬️native schema identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-json-rs:native-schema-check -- \"${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/json-native-schema"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05724
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🧭️router ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-ui-router-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05725
      }
    },
    {
      "name": "🧰️framework 🏃️process 🎯️exact Cargo 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-exact-cargo-laws",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05731
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🦀️native artifacts 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-native-artifacts",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05732
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🌐️Wasm build 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-wasm-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05733
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️Repo 🦀️dependency direction 🧱️gate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-cargo-dependency-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05739
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/cargo-direction"
      }
    },
    {
      "name": "🦑️Repo 🦀️dependency direction 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-dependency-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05738
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/cargo-direction"
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🧱️primitives",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:check-ui-primitives",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05726
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🌐️chrome i18n",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:check-chrome-i18n",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05727
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 📖️Storybook development",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:ui-storybook-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05728
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 📖️Storybook build",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:ui-storybook-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05729
      }
    },
  
    {
      "name": "🦑️Repo 🧱️fixture law ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-fixture-law-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05734
      }
    },
  
    {
      "name": "📇️Directory 🔏️lease fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:test-directory-lease-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05735
      }
    },
  
    {
      "name": "🌉️MCP ✅️approval fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp:test-approval-request-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05736
      }
    },
  
    {
      "name": "🌊️Flow 🏷️slider labels fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-flow-flow-rs:test-slider-labels-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05737
      }
    },
    {
      "name": "🌐️Locale 🏷️localized label fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-kernel:test-localized-label",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.0574
      }
    },
    {
      "name": "🦑️Repo 🌐️locale law ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-locale-law-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05741
      }
    },
    {
      "name": "🧬️Schema 🧺️mutation leaf registration 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-mutation-leaf-registration",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05744
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🌱️Value 🧩️neutral owner 🧪️products absent",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-value:test-neutral-owner --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05745
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🌱️Value 🦀️native 🧪️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05746
      }
    },
    {
      "name": "🦑️Repo 🦀️physical source 🧪️portable policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-direction --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05747
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️Repo 🦀️physical source 🛡️live direction",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-rust-source-direction --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05748
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "⚖️test🧬️validator🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-validator-rs:test-neutral-owner --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05749
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "⚖️document-http-check💻️os🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:document-http-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0575
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🌐️locale 🧪️strict portable contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale:test-contract --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/ui-locale"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05756
      }
    },
    {
      "name": "⚖️check🖱️ui🌐️locale🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale:check-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/ui-locale"
      },
      "presentation": {
        "group": "9_clean_architecture",
        "order": 901.05756
      }
    },
    {
      "name": "🦑️Repo 🧱️policy 🚀️fresh authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-dependency-policy-bootstrap --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05758
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🔗️rust-binding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-binding --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05759
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🔗️rust-binding🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-binding-native --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0576
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🧾️attributes",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-attributes --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05761
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🏘️scopes",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-scopes --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05762
      }
    },
    {
      "name": "🧪️test🦑️repo🧪️test🚷️discovery-boundaries",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test:test-discovery-boundaries --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05763
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🕸️graph",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-graph --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05764
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️scaffolding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-scaffolding --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05765
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️inventory",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-inventory --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05766
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️reachability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-reachability --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05767
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️type-origin",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-type-origin --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05768
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05769
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference🔁️revision",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference-revision --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0577
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference🔍️imports",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference-imports --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05771
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧱️rust-source-direction🔗️participation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-participation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05772
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧾️serialization🔣️json",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-canonical-json --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05773
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧹️normalization🏗️source-services",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-source-services --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05774
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧱️rust-source-direction🪵️root",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-roots --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05775
      }
    },
    {
      "name": "🧪️test🧰️framework🎠️kernel🫧️transient🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-transient:test-contract --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05776
      }
    },
    {
      "name": "🧪️test🧰️framework🫧️transient📦️retained",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-transient:test-retained --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05777
      }
    },
    {
      "name": "🧪️test🧰️framework🧬️schema✅️validator🧩️shape",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-validator-ts:test-shape --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05778
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite-source --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05779
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite-native --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057791
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057792
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🖊️dwg🎛️controlled-metadata",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-controlled-metadata --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0578
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio📖️pdf♻️recursive-retirement",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-pdf-rs:test-recursive-retirement --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05781
      }
    },
    {
      "name": "🧪️test🦑️repo🧱️rust🧫️fixture-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-fixture-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05782
      }
    },
    {
      "name": "🧪️test🧰️framework💻️os🔌️plugin🤝️cooperative-host",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:cooperative-host-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/cooperative-host"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05783
      }
    },
    {
      "name": "🧪️test🧰️framework💻️os🔌️plugin🤝️cooperative-host🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:cooperative-host-native-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/cooperative-host"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057831
      }
    },
    {
      "name": "🧪️test🌱️value🛬️portable",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test-portable --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05784
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache -- source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05785
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache -- native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057851
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057852
      }
    },
    {
      "name": "🧪️test🦑️repo🔐️pool-use-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-pool-use-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/pool-use-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05786
      }
    },
    {
      "name": "🧪️test🦑️repo🔔️deferred-wake-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-deferred-wake-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/deferred-wake-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05788
      }
    },
    {
      "name": "🧪️test🦑️repo📍️rust-family-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-family-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework/captured-family"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05789
      }
    },
    {
      "name": "🧪️test🦑️repo📍️rust-family-ownership🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-family-ownership-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework/captured-family"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057891
      }
    },
    {
      "name": "🧪️test🧬️schema🧱️neutrality",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-neutrality --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0579
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🧩️composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-composition --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-oracle"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05791
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-native-oracles --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-oracle-native"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05792
      }
    },
    {
      "name": "🧪️test🧬️schema📇️registry🦀️test-neutrality",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-registry-rs:test-neutrality --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05793
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05794
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-quick",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-quick --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05795
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-long",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-long --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05796
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-exhaustive",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-exhaustive --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05797
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05798
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-quick",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-quick --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05799
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-long",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-long --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.058
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-exhaustive",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-exhaustive --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05801
      }
    },
    {
      "name": "🧪️test📕️norm🧾️definition🧱️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-artifact-contract-rs:test-definition-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/norm-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05802
      }
    },
    {
      "name": "🧪️test🧩️puzzle🧵️retained📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-puzzle-retained-test:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-puzzle-retained"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05803
      }
    },
    {
      "name": "🧪️test🧩️puzzle🧵️retained🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-puzzle-retained-test:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-puzzle-retained"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05804
      }
    },
    {
      "name": "🧪️test🪐️space🧫️fixture📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-space-fixture-sources-test:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-space-fixture-sources"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05805
      }
    },
    {
      "name": "🧪️test🪐️space🧫️fixture🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-space-fixture-sources-test:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-space-fixture-sources"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05806
      }
    },
    {
      "name": "🧪️test🔌️plugin🏗️fixture🧬️interfaces",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:test-fixture-channel-interfaces --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/fixture-channel-interfaces"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05807
      }
    },
    {
      "name": "🧪️test📽️pptx📐️transform🔣️wire",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-pptx-rs:test-transform-wire --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/pptx-transform-wire"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05809
      }
    },
    {
      "name": "🧪️test🛠️tool🕸️rows🧬️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-tool-machine-rs:test-node-graph-row-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/node-graph-row-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05810
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🖊️drawing-reader",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-drawing-reader --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-drawing"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05811
      }
    },
    {
      "name": "🧪️test🌱️value🏷️type🧬️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test-type-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/value-type-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05813
      }
    },
    {
      "name": "🧪️test🖱️ui🎚️ring📬️press",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-control-commit --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-ui-neutrality/ring-press"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05815
      }
    },
    {
      "name": "🧪️test🗣️dsl🧱️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-dsl-rs:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05814
      }
    },
    {
      "name": "🧪️test📚️compiler🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-compiler-rs:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/compiler-native"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0582
      }
    },
    {
      "name": "🧪️test🗣️dsl🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-dsl-rs:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/dsl-native"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05819
      }
    },
    {
      "name": "🧪️test🌎️hub🧫️private-reader",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-private-reader --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/private-reader-preservation"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05817
      }
    },
    {
      "name": "🧪️test🌎️hub🖊️drawing🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-native-drawing-reader --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/lower-drawing-native-artifacts",
        "CARGO_TARGET_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/lower-drawing-target",
        "CARGO_BUILD_BUILD_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/lower-drawing-target/intermediate"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05818
      }
    },
    {
      "name": "🧪️test🌱️value🛬️decode🧩️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test-decode-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/decode-test-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05821
      }
    }
  ],
  "compounds": [
    {
      "name": "🧭️compound🖥️s⚛️react🌉️os-mcp",
      "configurations": [
        "🛠️dev🌉️os-mcp🌐️http",
        "🛠️dev🪐️space⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.16
      }
    },
    {
      "name": "🧭️compound🖥️s⚛️react🗄️os-hub",
      "configurations": [
        "🛠️dev🗄️os-hub",
        "🛠️dev🪐️space⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.15
      }
    },
    {
      "name": "🧭️compound🖥️s👥️users🗄️os-hub",
      "configurations": [
        "🛠️dev🗄️os-hub",
        "🛠️dev🪐️space👤️1⚛️react",
        "🛠️dev🪐️space👤️2⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.16
      }
    },
    {
      "name": "🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor",
      "configurations": [
        "🛠️dev🎓️teaching🛂️proctor",
        "🛠️dev🎓️teaching🏛️architecture❓️quiz"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 213.62
      }
    }
  ],
  "inputs": [
    {
      "id": "wgpuEmbeddedConfiguration",
      "type": "promptString",
      "description": "Ticket input configuration for actual embedded renderer acceptance",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️embedded-browser/🧊️wgpu.json"
    },
    {
      "id": "wgpuEmbeddedArtifacts",
      "type": "promptString",
      "description": "Ticket directory for embedded browser receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/embedded-browser"
    },
    {
      "id": "wgpuDockReactServe",
      "type": "promptString",
      "description": "React renderer host URL",
      "default": "http://127.0.0.1:7300/"
    },
    {
      "id": "wgpuDockServe",
      "type": "promptString",
      "description": "WGPU renderer host URL",
      "default": "http://127.0.0.1:7301/?plugin=puzzle3d"
    },
    {
      "id": "wgpuDockArtifacts",
      "type": "promptString",
      "description": "Ticket directory for Dock acceptance receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/dock-browser"
    },
    {
      "id": "wgpuMediaServe",
      "type": "promptString",
      "description": "WGPU stdio media viewer host URL",
      "default": "http://127.0.0.1:7303/?plugin=stdio-wav"
    },
    {
      "id": "wgpuMediaLocale",
      "type": "pickString",
      "description": "Explicit acceptance language",
      "options": [
        "en",
        "de"
      ],
      "default": "en"
    },
    {
      "id": "wgpuMediaArtifacts",
      "type": "promptString",
      "description": "Ticket directory for media acceptance receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/media-browser"
    },
    {
      "id": "rendererParityRenderer",
      "type": "pickString",
      "description": "Renderer comparison",
      "options": [
        "paired",
        "react",
        "wgpu"
      ],
      "default": "paired"
    },
    {
      "id": "rendererParityJourneyArtifacts",
      "type": "promptString",
      "description": "Ticket directory for shell interaction receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/shell-interaction"
    },
    {
      "id": "nativeScaleRegistry",
      "type": "promptString",
      "description": "Native scale registry JSON path",
      "default": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🤖️generated/📇️registry/🔣️.json"
    },
    {
      "id": "gisVerification",
      "type": "pickString",
      "description": "GIS verification",
      "options": [
        "imports",
        "watcher",
        "map",
        "graph"
      ],
      "default": "map"
    },
    {
      "id": "nxCacheTicket",
      "type": "promptString",
      "description": "Active ticket directory for Nx diagnostics"
    },
    {
      "id": "artifactPackageProject",
      "type": "promptString",
      "description": "Artifact package Nx project, for example @semio-tech/stdio-pdf-rs"
    },
    {
      "id": "artifactPackageTarget",
      "type": "pickString",
      "description": "Artifact package target",
      "options": [
        "build",
        "check",
        "test"
      ]
    },
    {
      "id": "catalogFreshBuildRoot",
      "type": "promptString",
      "description": "Absolute fresh plugin catalog build root"
    },
    {
      "id": "acceptanceHubUrl",
      "type": "promptString",
      "description": "Hub under acceptance (origin)",
      "default": "http://127.0.0.1:8787"
    },
    {
      "id": "acceptanceServeUrl",
      "type": "promptString",
      "description": "s React serve joined to that hub",
      "default": "http://127.0.0.1:6070/"
    },
    {
      "id": "acceptanceLocalServeUrl",
      "type": "promptString",
      "description": "Local-only s React serve (every plugin loaded; launch row 🛠️dev🪐️space⚛️react🔒local-only)",
      "default": "http://127.0.0.1:6070/"
    },
    {
      "id": "acceptanceHubAdminCapability",
      "type": "promptString",
      "description": "The hub launcher's admin-capability.json (0600) for gates that read the hub's connection census",
      "default": ""
    },
    {
      "id": "acceptanceUsers",
      "type": "promptString",
      "description": "Optional JSON file {\"users\":[{\"email\",\"password\"}…]} with the hub's test users (empty: each gate's development users)",
      "default": ""
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️ownership🌎️hub🧩️compositions🟦️",
      "command": "bun nx run @semio-tech/hub-compositions:ownership",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "4_gate", "order": 900.06 }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "🧪️native🌎️hub🧩️compositions🦀️",
      "command": "bun nx run @semio-tech/hub-compositions:native",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "4_gate", "order": 900.07 }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️check🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.08
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.09
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-quick🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-quick",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-long🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-long",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.11
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-exhaustive🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-exhaustive",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.12
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️mutation-verb-vocabulary-check🧬️mutation-verbs🟦️",
      "command": "bun nx run @semio-tech/framework-os:mutation-verb-vocabulary-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.13
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️verify-inference-client🧬️gis-gismap🦀️",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-client",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.14
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️verify-inference-client-native🧬️gis-gismap🦀️",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-client-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.15
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️contract-check📦️component-deployment🟦️",
      "command": "bun nx run @semio-tech/component-deployment-contract:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.16
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️generation3d-semantic-wire-source🧊️procedural🦀️",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:semantic-wire-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.17
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️generation3d-semantic-wire-native🧊️procedural🦀️",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:semantic-wire-check -- native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1701
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-mcp-composition-source🧩️s🟦️",
      "command": "bun nx run @semio-tech/s-services-native:mcp-composition-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.18
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-mcp-untrusted-content🧩️s🟦️",
      "command": "bun nx run @semio-tech/s-services-native:untrusted-content-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1801
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-fixture-ownership🧩️s🦀️",
      "command": "bun nx run @semio-tech/s-services-native:fixture-ownership-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1802
      }
    }
,
    {
      "name": "⚖️check-wgpu-boot-cache-inputs🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:check-boot-cache-inputs --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1803
      }
    },
    {
      "id": "stdioRemovalArtifacts",
      "type": "promptString",
      "description": "Ticket output directory containing the retained copied workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio"
    },
    {
      "id": "stdioRemovalSnapshot",
      "type": "promptString",
      "description": "Retained workspace copy with only AVI absent, relative to the workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/parent-avi-removal"
    },
    {
      "id": "graphContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned graph contract artifacts directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/graph-contract"
    },
    {
      "id": "svgVideoArtifacts",
      "type": "promptString",
      "description": "Ticket-owned SVG video artifacts directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/svg-video"
    },
    {
      "id": "processContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned framework process contract output directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/process-contract"
    },
    {
      "id": "identityContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned identity contract output directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/grapheme-removal"
    }
  ],

  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated
  // output); see 🚀️launch/🟦️.ts readSeed() for the exact split contract this marker line supports.
  "devLaunchers": {
    "aggregator": {
      "namePrefix": "♻️mit-bestand🧩️puzzle🧊️3d",
      "order": 212
    },
    "animate": {
      "namePrefix": "🎞️animate🎬️presentation",
      "order": 170,
      "wgpuOrder": 170.1
    },
    "architect": {
      "namePrefix": "🏛️architect🏛️program",
      "order": 142,
      "wgpuOrder": 142.1
    },
    "aussuchen": {
      "namePrefix": "♻️mit-bestand🪵️sourcing🗂️curation",
      "order": 216
    },
    "bearbeiten": {
      "namePrefix": "♻️mit-bestand🏭️process🧊️process3d",
      "order": 217
    },
    "block2d": {
      "namePrefix": "🧱️block◻️2d",
      "order": 261,
      "wgpuOrder": 261.1
    },
    "block3d": {
      "namePrefix": "🧱️block🧊️3d",
      "order": 262,
      "wgpuOrder": 262.1
    },
    "block5d": {
      "namePrefix": "🧱️block🖐️5d",
      "order": 263,
      "wgpuOrder": 263.1
    },
    "cad": {
      "namePrefix": "📐️cad",
      "order": 10,
      "wgpuOrder": 10.1
    },
    "dag": {
      "namePrefix": "🕸️dag",
      "order": 140,
      "wgpuOrder": 140.1
    },
    "draw": {
      "namePrefix": "🖍️draw🖍️drawing",
      "order": 387,
      "wgpuOrder": 387.1
    },
    "fem2d": {
      "namePrefix": "🏗️fem◻️2d",
      "order": 392,
      "wgpuOrder": 392.1
    },
    "fem3d": {
      "namePrefix": "🏗️fem🧊️3d",
      "order": 392.3,
      "wgpuOrder": 392.4
    },
    "flow": {
      "namePrefix": "🌊️flow",
      "order": 150,
      "wgpuOrder": 150.1
    },
    "forms": {
      "namePrefix": "📋️forms",
      "order": 385,
      "wgpuOrder": 385.1
    },
    "generator": {
      "namePrefix": "♻️mit-bestand🌀️procedural🧊️generation3d",
      "order": 214
    },
    "gis2d": {
      "namePrefix": "🌍️gis🗺️gismap",
      "order": 160,
      "wgpuOrder": 160.1
    },
    "gis3d": {
      "namePrefix": "🌍️gis🏔️gisterrain",
      "order": 160.3,
      "wgpuOrder": 160.4
    },
    "imperative": {
      "namePrefix": "📜️imperative📜️procedure",
      "order": 155,
      "wgpuOrder": 155.1
    },
    "koordinator": {
      "namePrefix": "♻️mit-bestand📐️cad",
      "order": 215
    },
    "layout": {
      "namePrefix": "📏️layout",
      "order": 158,
      "wgpuOrder": 158.1
    },
    "lowpoly": {
      "namePrefix": "💠️lowpoly",
      "order": 157,
      "wgpuOrder": 157.1
    },
    "mathematical": {
      "namePrefix": "➗️mathematical➗️equation",
      "order": 141,
      "wgpuOrder": 141.1
    },
    "note": {
      "namePrefix": "🗒️note",
      "order": 388,
      "wgpuOrder": 388.1
    },
    "bitmap": {
      "namePrefix": "🀄️wfc🖼️bitmap",
      "order": 201,
      "wgpuOrder": 201.1
    },
    "grid2d": {
      "namePrefix": "🀄️wfc🔲️grid2d",
      "order": 202,
      "wgpuOrder": 202.1
    },
    "wfc2d": {
      "namePrefix": "🀄️wfc◻️2d",
      "order": 203,
      "wgpuOrder": 203.1
    },
    "grid3d": {
      "namePrefix": "🀄️wfc🧱️grid3d",
      "order": 204,
      "wgpuOrder": 204.1
    },
    "wfc3d": {
      "namePrefix": "🀄️wfc🧊️3d",
      "order": 205,
      "wgpuOrder": 205.1
    },
    "generation2d": {
      "namePrefix": "🌀️procedural🌀️generation2d",
      "order": 180,
      "wgpuOrder": 180.1
    },
    "generation3d": {
      "namePrefix": "🌀️procedural🧊️generation3d",
      "order": 190,
      "wgpuOrder": 190.1
    },
    "process3d": {
      "namePrefix": "🏭️process🧊️process3d",
      "order": 195,
      "wgpuOrder": 195.1
    },
    "puzzle2d": {
      "namePrefix": "🧩️puzzle◻️2d",
      "order": 220,
      "wgpuOrder": 220.1
    },
    "puzzle3d": {
      "namePrefix": "🧩️puzzle🧊️3d",
      "order": 230,
      "wgpuOrder": 230.1
    },
    "puzzle5d": {
      "namePrefix": "🧩️puzzle🖐️5d",
      "order": 250,
      "wgpuOrder": 250.1
    },
    "raster": {
      "namePrefix": "🖨️raster",
      "order": 386,
      "wgpuOrder": 386.1
    },
    "reasoning-wires": {
      "namePrefix": "💡️reasoning🔌️wires",
      "order": 270,
      "wgpuOrder": 270.1
    },
    "remodel": {
      "namePrefix": "📸️remodel📸️remodeling",
      "order": 389,
      "wgpuOrder": 389.1
    },
    "s": {
      "namePrefix": "🪐️space",
      "order": 386.2,
      "wgpuOrder": 386,
      "env": {
        "S_HUB_URL": "http://127.0.0.1:8787",
        "S_DATA_DIR": "${workspaceFolder}/.🧬semio/🔗space/s-dev"
      },
      "users": {
        "namePrefixPattern": "🖥️s👤️{N}",
        "emailPattern": "user{N}@semio.dev",
        "env": {
          "S_HUB_URL": "http://127.0.0.1:8787",
          "S_DATA_DIR": "${workspaceFolder}/.🧬semio/🔗space/s-user{N}"
        }
      }
    },
    "sequence": {
      "namePrefix": "🎬️sequence",
      "order": 156,
      "wgpuOrder": 156.1
    },
    "shooting": {
      "namePrefix": "🎥️shooting",
      "order": 290,
      "wgpuOrder": 290.1
    },
    "sourcing": {
      "namePrefix": "🪵️sourcing🗂️curation",
      "order": 196,
      "wgpuOrder": 196.1
    },
    "trinity-jack": {
      "namePrefix": "🔱️trinity🔌️jack",
      "order": 360,
      "wgpuOrder": 360.1
    },
    "trinity-rewriting": {
      "namePrefix": "🔱️trinity♻️rewriting",
      "order": 380,
      "wgpuOrder": 380.1
    },
    "vcs": {
      "namePrefix": "🌿️vcs",
      "order": 385,
      "wgpuOrder": 385.1
    },
    "verfolgen": {
      "namePrefix": "♻️mit-bestand🌍️gis🗺️gismap",
      "order": 218
    },
    "writer": {
      "namePrefix": "✒️writer",
      "order": 387,
      "wgpuOrder": 387.1
    }
  },
  // 📋️projectLaunchers — how every declared `📋️project.json` target becomes a launch row (not part of the generated
  // output); see 🚀️launch/🟦️.ts `renderProjectTargetLaunchers` for the contract. A curated row above that runs a target wins.
  "projectLaunchers": {
    "familyMinimumProjects": 3,
    "familyEmoji": "📋️",
    "fallbackClass": "run",
    "classes": [
      {
        "id": "dev",
        "emoji": "🛠️",
        "group": "3_dev",
        "orderBase": 900,
        "tokens": [
          "dev",
          "serve",
          "start",
          "watch",
          "activate",
          "open",
          "launch",
          "inspect",
          "demo",
          "playground",
          "attach"
        ]
      },
      {
        "id": "build",
        "emoji": "📦️",
        "group": "4_build",
        "orderBase": 900,
        "tokens": [
          "build",
          "package",
          "wasm",
          "publish",
          "release",
          "bundle",
          "generate",
          "generator",
          "typegen",
          "codegen",
          "preview",
          "deps",
          "fonts",
          "prepare",
          "materialize",
          "install",
          "compile",
          "sign",
          "deploy",
          "bootstrap",
          "clean",
          "prune",
          "restage",
          "rebuild",
          "setup",
          "format",
          "fix",
          "regenerate"
        ]
      },
      {
        "id": "gate",
        "emoji": "⚖️",
        "group": "4_gate",
        "orderBase": 900,
        "tokens": [
          "test",
          "check",
          "verify",
          "lint",
          "typecheck",
          "oracle",
          "native",
          "e2e",
          "probe",
          "audit",
          "census",
          "bench",
          "parity",
          "law",
          "laws",
          "drill",
          "smoke",
          "conformance",
          "contract",
          "validate",
          "scan",
          "report",
          "doctor",
          "discover",
          "inventory",
          "metrics"
        ]
      },
      {
        "id": "run",
        "emoji": "▶️",
        "group": "3_dev",
        "orderBase": 950,
        "tokens": [
          "run"
        ]
      }
    ],
    "languageSegments": {
      "🦀️rust": "🦀️",
      "🟦️typescript": "🟦️",
      "🐍️python": "🐍️"
    },
    "transparentSegments": [
      "📦️packages"
    ],
    "skipDirectories": [
      "node_modules",
      "dist",
      "target",
      "temp",
      "pkg",
      "storybook-static",
      "🤖️generated",
      "🗑️generated",
      "🎫️tickets"
    ]
  }
}

```

## Explicit Worker Launch Source GREEN

Registered current worker route passed all five portable laws and the independent JSONC parser comparison after adding authored seed900.05816. Exact console terminal counts and uncached duration are retained in `worker-portable-launch-green.log`; the earlier missing-seed run is genuine4passed/1failed/29assertions/121msBun/10sNx. The seed supplies the caller-owned ticket lifecycle output path and sits before seed17 in group9_gates. Ordinary generated launch production remains Root-owned; no generated bytes were handpatched.

## Current Normal Cargo Direction Route Inspection

The original registered route is `bun nx run @semio-tech/repo-lib:lint-cargo-dependency-direction --skip-nx-cache`. Its actual producer inventories every authored declaration kind independently, then collects each actual discovered workspace with ordinary `cargo metadata --format-version 1 --no-deps --offline --locked --manifest-path ...`; its cumulative metadata deadline remains30seconds. It requires metadata/inventory identity before reporting all strict physical/semantic edges and classifications. No current normal Cargo direction verdict was run while DSL source readiness is withheld, and a manifest-only observation cannot grant this route. It remains queued after ordinary coherent preparation and current metadata readiness.

## Unadmitted Guard Boundary Review

The fixture first-read observation inspects optional state before admission; the actual guard requires a ready Ui and panics with the named unresolved fault if dereferenced before admission. That panic occurs while a MutexGuard is held; ordinary Interpreter WorkerCell intentionally preserves its prior poisoned-lock refusal behavior, unlike the scene cell which explicitly recovers poison. The authored native refusal law verifies the named fault and stops there. It does not prove later admission recovers after a programmer-error dereference. Production boot admits locale before any intended Ui access. If resumable admission following a rejected guard access is required, that is an additional lifecycle obligation rather than a proved outcome. Current original full Renderer runtime remains pending.

## Current Worker Source Fence Before DSL Handoff

Observed 2026-10-02T18:02:47.942Z. Fourteen current owned production, schema, fixture, native/portable law and ordinary package-router inputs are recorded in `current-native-worker/worker-current-source-fence.json` with physical byte lengths and SHA256. This is a preparation observation only. Original full Renderer compile/runtime remains held until coherent DSL readiness and fresh ordinary prepare/locked metadata. Whole launch seed is deliberately not treated as a stable Worker fence while Root edits unrelated authored entries; Worker seed16 is independently required by the actually passing portable law.

## Original Normal Cargo Direction Fresh Epoch

DSL High declared coherent SourceREADY and held owned Rust/Cargo edits for this bounded epoch. Before the original uncached `@semio-tech/repo-lib:lint-cargo-dependency-direction` route, independently hashed 296 current authored manifests plus workspace lockfiles and the unchanged policy/producer inputs; current inventory 292 packages across 4 discovered workspaces. Before snapshot is `current-native-worker/cargo-direction-before.json`. The cumulative30s metadata deadline, all normal/dev/build/optional/platform declarations and normal workspace preparation remain unchanged. Terminal verdict and after hashes are pending.

## Actual Full Current Value Runtime and Exact Original Roster

Entire original unfiltered `@semio-tech/value-rs:test` replay2 genuinely passed115run/115pass/0skipped, compile2.02seconds, runtime0.499seconds and5.1secondsuncachedNx. Original long profile and selectors retained. The actually executed native binary was independently inspected read-only with libtest list (no new laws executed); its115names include both lower decode laws exactly once. The earlier truly executed113binary was also inspected, and exact set comparison proves0originalremoved and precisely2added lowercontrollednames. This is actual current discovery/runtime, not an inferred count.

All58actualexecuted compiler-owned input byte lengths/BLAKE3 independently match current physical sources, including the lowercontrolled law and closedfixtures. All19Root fresh SOURCE READY2 input hashes remained unchanged across launch/runtime. Receipt/sourceclosure/rostercomparison is `current-native-worker/value-decode-current-runtime-source-origins-2.json`, `value-original-to-current-roster-comparison.json`, both retained roster textfiles and `value-decode-native-full-2.log`. The prior Value compileGREEN/temporary metadata-cycleRED remains preserved as replay1. Root may now retire only the exact duplicate higher mount as separately authorized, with a fresh consumer epoch before higher compiles.

## Prior Complete UI engine Receipt Currentness

Freshly rechecked every440 own source input from the earlier actually executed complete UI engine binary against the physical current bytes. 7 current input hashes differ; every changed path is retained in `current-native-worker/ui-engine-prior-executed-source-currentness.json`. Earlier full runtime remains a genuine historical receipt but cannot grant the current changed own source epoch.

- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️conformance-corpus/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/♿️retained-section-accessibility/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs

## Prior Complete UI default Receipt Currentness

Freshly rechecked every310 own source input from the earlier actually executed complete UI default binary against the physical current bytes. 1 current input hashes differ; every changed path is retained in `current-native-worker/ui-default-prior-executed-source-currentness.json`. Earlier full runtime remains a genuine historical receipt but cannot grant the current changed own source epoch.

- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs

### Root Type and Higher Mount Cut: Fresh Native Epoch

All43Root handed-off source inputs independently matched current physical lengths/SHA256 before launch. Prior High2,473input fence has3later gaps: authoredlaunchseed and separate mathematical/equation plusStdio/zip nativeSQLite laws; those are explicitly not treated as unchanged globalinputs. Fresh unchanged normal workspace dependency preparation passed6.9suncachedNx; existing complete locked metadata route passed3.4suncachedNx. Entire unfiltered original Value target replay3 with the current embeddedTypefixture now launched; no runtime count yet. Nativequeue retains fullValue, lowerDSL, Compiler, bothcurrentoriginalUI targets andRenderer before the higher/sweep/oracle/deletion obligations.

## Full Value Current Type Fixture Epoch Runtime

Fresh unfiltered unchanged Value replay3 genuinely passed115/115/zero skipped, compile3.66s/runtime0.721s/uncachedNx8.6s in original long profile. Actual executed binary roster is exactly the preceding115names with zero additions/removals, retaining every original113plus2lowerdecode laws. All58current compiler-owned input lengths/BLAKE3 match, including the new neutral Typefixture. All43Root handed-off source input hashes remain unchanged through terminal. Current source/roster evidence: `value-type-cut-current-runtime-source-origins-3.json`; native log `value-type-cut-native-full-3.log`. No old embeddedfixture binary is reused to grant this epoch.

## Complete Lower DSL Native Runtime

Original complete registered lowerDSL target actually passed84/84/0skipped across2binaries, original fundamental profile, runtime0.136s/compile2.11s/uncachedNx4.7s. Actual executed binary read-only rosters prove82librarylaws plus2publicgolden integrationlaws. All17 compiler-owned input rows across both binaries independently match current physical lengths/BLAKE3, with perbinary fullrows/lawnames in `dsl-current-runtime-source-origins-1.json`. Native log `dsl-native-full-1.log`. Compiler whole original library target is now launched, session98638; no Compiler runtime verdict is inferred.

## Complete Original Compiler Library Runtime

Full registered original library scope actually selected/run/passed54laws, zero skipped, including exact17syntaxlaws (the target was not narrowed to those17). Fundamental profile retained, compile2.69s/runtime0.091s/uncachedNx8.1s. Actual executed roster and all19 current rustc-owned input lengths/BLAKE3 verified, in `compiler-current-runtime-source-origins-1.json`; console `compiler-native-full-1.log`. Full original current UI default/engine serial pair is next, followed unchanged Renderer target.

### Independent DSL Original Roster Binding

High completed a source-only independent binding of actual executed82library names to immutable original lexer38+grammar39+literal3+idiom2. Every current original law source byte reconstructs from the authored original and owned transforms; exact2publicgolden names and all17compiled inputs match. Receipt `goal-lexical/dsl-native-original-roster-binding.json` complements actual runtime84/84/0skipped and the Native-owned executed binary source/roster rows. Compiler actualwhole54law runtime includes17syntax, rather than a narrowed17selection. Both originalcomplete UI targets currentlyrunserially underunchangedlongprofile; Renderer fulltargetremainsnext, no UI/Renderer runtime verdict yet.

## Actual Renderer Replay3 Canonical Locale Frontier

Full original unchanged Renderer replay3 terminal compilerRED3E0603, Nx3minutes, runtime not reached. Every diagnostic points only to ownnew additive locale-retention law spelling ui_wgpu::wgpu::Locale, now private aftercanonicalLocale extraction; production declaration/admission code compiles past original212and16diagnostic frontiers. Correct direct semio_framework_ui_locale::Locale alreadyownnormalmanifestdependency is required. Only three provider tokens in ownadditive test change, actual En/De values/loop/allassertions remain unchanged; fullbefore native test captured below. No public compatibility reexport or original test/body change.

```rust
use super::*;

struct NonDefaultState {
    value: u32,
}

#[test]
fn worker_cell_owned_initializer_supports_non_default_state_and_retains_one_instance() {
    let cell = WorkerCell::new(|| NonDefaultState { value: 73 });
    assert_eq!(cell.borrow().value, 73);
    cell.borrow_mut().value = 109;
    assert_eq!(cell.borrow().value, 109);
    assert!(std::ptr::eq(cell.state(), cell.state()));
}

#[test]
fn worker_cell_test_thread_states_are_isolated_by_cell_address_and_thread_identity() {
    static CELL: WorkerCell<NonDefaultState> = WorkerCell::new(|| NonDefaultState { value: 17 });
    CELL.borrow_mut().value = 33;
    let child = std::thread::spawn(|| {
        assert_eq!(CELL.borrow().value, 17);
        CELL.borrow_mut().value = 88;
        CELL.borrow().value
    });
    assert_eq!(child.join().unwrap(), 88);
    assert_eq!(CELL.borrow().value, 33);
}

#[test]
fn retained_ui_unadmitted_read_refuses_with_the_named_locale_fault() {
    let cell = UiEngineCell(WorkerCell::new(|| None));
    assert!(cell.0.borrow().is_none());
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let engine = cell.borrow();
        let _ = std::ops::Deref::deref(&engine);
    }))
    .expect_err("unadmitted retained UI cannot be read");
    let message = refusal.downcast_ref::<String>().map(String::as_str).or_else(|| refusal.downcast_ref::<&str>().copied());
    assert_eq!(message, Some(UI_ENGINE_LOCALE_UNRESOLVED));
}

#[test]
fn retained_ui_locale_readmission_preserves_the_actual_surface_tree_and_revision() {
    let node = UiNode::Text(ui_wgpu::wgpu::UiTextNode {
        value: semio_framework_ui_locale::Label::data("retained document"),
        emphasize: None,
        data_attributes: None,
        presence: ui_wgpu::wgpu::UiPresence::default(),
        menu: None,
    });
    install_ui_engine_locale(ui_wgpu::wgpu::Locale::De);
    UI_ENGINE.with(|cell| cell.borrow_mut().apply_tree("locale-owner", &node));
    let before = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        (engine.surface_token("locale-owner"), engine.tree_revision("locale-owner"), engine.tree("locale-owner").unwrap().root)
    });
    assert!(before.0.is_some());
    assert!(before.2.is_some());
    for locale in [ui_wgpu::wgpu::Locale::En, ui_wgpu::wgpu::Locale::De] {
        install_ui_engine_locale(locale);
        UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            assert_eq!((engine.surface_token("locale-owner"), engine.tree_revision("locale-owner"), engine.tree("locale-owner").unwrap().root), before);
            assert_eq!(engine.tree("locale-owner").unwrap().node(before.2.unwrap()).unwrap().spec.0, node);
        });
    }
}

```

## Typed Seed Admission

Whole Renderer React TypeScript compilation exposed TS18046 in the additive worker launch proof: Bun.JSONC.parse rightly returns unknown. Add a real object/array admission guard before accessing configurations, preserving every existing assertion and fullsourceinverse. Before:

```json
{
  "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🟦️.ts",
  "before": "import { test, expect } from \"bun:test\";\nimport { readFileSync, mkdirSync, writeFileSync } from \"node:fs\";\nimport { resolve } from \"node:path\";\nimport { createRequire } from \"node:module\";\nimport Ajv from \"ajv\";\nimport { createAdmittedWorkerCell, createWorkerCell } from \"../🟦️.ts\";\nconst require = createRequire(import.meta.url);\nconst memoize: (factory: (owner: string) => State) => ((owner: string) => State) = require(\"lodash/memoize\");\ntype State = { locale: string; retained: string[]; constructions: number };\nconst fixture = JSON.parse(readFileSync(resolve(import.meta.dir, \"../🧫️fixtures/🔣️.json\"), \"utf8\"));\nconst interpreter = resolve(import.meta.dir, \"../../🎯️targets/🧊️wgpu/🦀️.rs\");\n\ntest(\"closed independent lifecycle corpus retains exact admission and thread identities\", () => {\n  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(resolve(import.meta.dir, \"../🧬️schema/🔣️.json\"), \"utf8\")));\n  expect(validate(fixture)).toBe(true);\n  expect(validate({ ...fixture, schemaVersion: 2 })).toBe(false);\n  const actual = [];\n  for (const row of fixture.cases) {\n    const cell = createAdmittedWorkerCell<State>();\n    const ready = new Set<string>();\n    const reference = memoize(() => ({ locale: \"\", retained: [], constructions: 1 }));\n    const observations = [], oracle = [];\n    for (const operation of row.operations) {\n      if (operation.action === \"admit\") {\n        cell.admit(operation.owner, () => ({ locale: operation.locale, retained: [], constructions: 1 }), state => { state.locale = operation.locale; });\n        ready.add(operation.owner); reference(operation.owner).locale = operation.locale;\n      } else if (operation.action === \"retain\") {\n        cell.read(operation.owner)!.retained.push(operation.value); reference(operation.owner).retained.push(operation.value);\n      }\n      const state = cell.read(operation.owner), expected = ready.has(operation.owner) ? reference(operation.owner) : undefined;\n      observations.push(state ? { owner: operation.owner, state: \"ready\", ...structuredClone(state) } : { owner: operation.owner, state: \"unadmitted\" });\n      oracle.push(expected ? { owner: operation.owner, state: \"ready\", ...structuredClone(expected) } : { owner: operation.owner, state: \"unadmitted\" });\n    }\n    expect(observations).toEqual(row.expected); expect(oracle).toEqual(row.expected); actual.push({ id: row.id, observations });\n  }\n  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;\n  if (!output) throw new Error(\"Caller-owned SEMIO_TEST_ARTIFACT_DIR is required\");\n  mkdirSync(output, { recursive: true }); writeFileSync(resolve(output, \"worker-cell-lifecycle.json\"), JSON.stringify(actual, null, 2));\n});\n\ntest(\"nonDefault construction is lazy and scoped rather than implicit\", () => {\n  let calls = 0; const cell = createWorkerCell(() => ({ secret: ++calls }));\n  expect(calls).toBe(0); expect(cell.read(\"a\")).toBe(cell.read(\"a\"));\n  expect(cell.read(\"b\").secret).toBe(2); expect(cell.read(\"a\").secret).toBe(1);\n});\n\ntest(\"actual Rust worker source requires owned factories and explicit locale admission\", () => {\n  const source = readFileSync(interpreter, \"utf8\");\n  expect(source).not.toMatch(/impl<T: Default|test_worker_cell<T: Default/);\n  expect(source.includes(\"initialize: fn() -> T\")).toBe(true);\n  expect(source.includes(\"get_or_init(|| Mutex::new((self.initialize)()))\")).toBe(true);\n  expect(source.includes(\"test_worker_cell::<T>(std::ptr::from_ref(self).addr(), self.initialize)\")).toBe(true);\n  expect(source.includes(\"UiEngineCell(WorkerCell<Option<ui_wgpu::wgpu::Ui>>)\")).toBe(true);\n  expect(source.match(/fn initial_ui_engine\\(\\)[\\s\\S]*?\\n}/)?.[0]).toMatch(/#\\[cfg\\(test\\)\\]\\s+return Some\\(ui_wgpu::wgpu::Ui::new\\(TEST_UI_ENGINE_LOCALE\\)\\);\\s+#\\[cfg\\(not\\(test\\)\\)\\]\\s+None/);\n  expect(source.includes(\"self.0.as_ref().expect(UI_ENGINE_LOCALE_UNRESOLVED)\")).toBe(true);\n  expect(source.includes(\"self.0.as_mut().expect(UI_ENGINE_LOCALE_UNRESOLVED)\")).toBe(true);\n  const admission = source.match(/pub\\(crate\\) fn install_ui_engine_locale\\([^)]*\\)[\\s\\S]*?\\n}/)?.[0];\n  expect(admission).toMatch(/Some\\(ui\\) => ui.set_locale\\(locale\\),/);\n  expect(admission).toMatch(/None => \\*engine = Some\\(ui_wgpu::wgpu::Ui::new\\(locale\\)\\),/);\n  expect(source.includes('#[path = \"../../🧵️worker-cell/🧪️tests/🦀️.rs\"]')).toBe(true);\n});\n\ntest(\"actual Shell and Ui locale updates retain the existing engine and Scenes cell contract\", () => {\n  const shell = readFileSync(resolve(import.meta.dir, \"../../../🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs\"), \"utf8\");\n  expect(shell).toMatch(/pub fn new\\(plugins: Vec<ProgramBridgeEntry>, plugin_filter: String, locale: Locale, terminology: Terminology\\) -> Self \\{\\s+crate::interpreter::install_ui_engine_locale\\(locale\\);/);\n  expect(shell.match(/crate::interpreter::install_ui_engine_locale\\(self.active_locale\\(\\)\\);/g)?.length).toBe(3);\n  const engine = readFileSync(resolve(import.meta.dir, \"../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs\"), \"utf8\");\n  expect(engine.match(/pub fn set_locale\\([^)]*\\)[\\s\\S]*?\\n    }/)?.[0]).toMatch(/\\{\\s+self.shell.set_locale\\(locale\\);\\s+}/);\n  const scenes = readFileSync(resolve(import.meta.dir, \"../../../🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs\"), \"utf8\");\n  expect(scenes.includes(\"OnceLock<Mutex<RefCell<T>>>\")).toBe(true);\n  expect(scenes.includes(\"std::sync::PoisonError::into_inner\")).toBe(true);\n  expect(scenes.includes(\"|| RefCell::new(T::default())\")).toBe(true);\n});\n\n\ntest(\"the authored worker launch supplies its explicit caller-owned lifecycle output\", () => {\n  const source = readFileSync(resolve(import.meta.dir, \"../../../../../../../../../../.vscode/🧩️launch.seed.jsonc\"), \"utf8\");\n  const seed = Bun.JSONC.parse(source);\n  expect(seed).toEqual(require(\"jsonc-parser\").parse(source));\n  const launches = seed.configurations.filter((row: { command?: string }) => row.command === \"bun nx run @semio-tech/framework-renderer-wgpu:test-worker-cell --skip-nx-cache\");\n  expect(launches).toHaveLength(1);\n  expect(launches[0].presentation.order).toBe(900.05816);\n  expect(launches[0].env.SEMIO_TEST_ARTIFACT_DIR).toBe(\"${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-worker/portable-launch-artifacts\");\n});\n"
}
```


## Whole Native Renderer Epoch Five Upstream Compiler Terminal

The unchanged complete `test-wgpu-unit` target exited1 after7m47sNx, with one upstreamOSKernel E0271 at lower SQLite transfer line135. No Renderer native discovery/runtime or Worker test executed. Cargo completed remaining in-flight jobs after reporting the error; no active compiler was restarted from an expired observation.

The retained failedKernel compiler `.d` emitted at03:31:31UTC has152 checksum rows. Three current differences affect IOaggregate, the lower snapshot root and its newly/currently mounted transfer source. The previous whole SQLite32 binary's actual14inputs did not include transfer, and the genuine earlier39-source capture contains no transfer file; its GREEN cannot prove this new producer. Complete current observed IO sources and available prior provenance are preserved in `renderer-worker-epoch-5-current-io-and-ts-independent-deltas.json`. No unavailable complete failed-transfer before body or exact historical inverse is invented.

Before any owned write, the current transfer already contained `previous:Option<usize>` and `self.pages[previous].as_mut_slice()`, independently implementing the necessary scalar/slice binding. This lane authored no transfer repair and preserves the current work with unknown attribution. Further observed transfer growth during inspection is retained rather than rolled back. The old3128source fence had one lateWorld3dHostTSdifference and omitted newly introduced dependency source paths; no global-current verdict follows.

Fresh bounded epochSix captures3280fullinputs65.3MB by adding all152actualfailedKernel origins to the originalRenderer/Value/config scopes. Its replay keeps the complete originalselector/profile/features/deadline. A fresh lockedmetadata invocation is running before the unchanged full replay. The four Worker native outcomes remain pending.
