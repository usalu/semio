# Example Switcher Data Source Analysis

## 1. React Shell Dropdown Data Source

**Source**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx:5785`

The navbar example dropdown uses **`activePluginManifest?.examples`** — the manifest's `examples` field:

```typescript
return (activePluginManifest?.examples ?? [])
  .filter((example) => example.appId === appId)
  .map((example) => ({
    id: example.id,
    label: resolveAppLabel(appLabelsOverlay, "example", example.id, ..),
    icon: "file" as IconName,
  }));
```

**Hidden condition** (line 5813): Dropdown renders `null` when `exampleOptions.length === 0 || locks.exampleId || (hostMode && session.app.id === landingAppId)`

## 2. Rust Plugin Builder API

**Schema**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

- **App struct** (line 7989): `pub struct App { pub definition: AppDefinition, pub examples: Vec<ExampleDefinition> }`
- **ExampleSource** (lines 7863–7922): Builder for single example:
  ```rust
  pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, 
    document_json: impl Into<String>, icon_id: impl Into<IconName>) -> Self
  ```
- **App example method** (lines 8010–8013):
  ```rust
  pub async fn example_source(mut self, source: impl Into<ExampleSource>) -> Self {
    self.examples.push(ExampleDefinition::from(source.into()));
    self
  }
  ```

**Problem**: `EditorBuilder` (line 4772–4836) has **no `.example()` method**. `AppBuilder::build_definition()` or `AppBuilder::from_builder()` return apps with empty `examples: Vec::new()` (lines 8001, 8006).

## 3. Example Fixtures vs Playground Session

- **App-level examples** (lines 8010–8013): Populated via `.example_source(ExampleSource::new(...))` chain on App post-build
- **Playground session examples** (playgrounds.json): `"🎬️demo-session"` is a **dev-shell fixture** in `📚️examples/🎬️demo-session/` of each artifact editor, unrelated to app examples — loaded via artifact's own example module (e.g., writer's `examples::app_writer_demo_session`)

## 4. Missing Piece for Gen3d Example Switcher

**Generation3d** has 8 example fixtures (ids at `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:275–282`):
- `"hexagonal-mushroom-column"` through `"box-shell-preview"`

**Currently**:
- Hardcoded in `setActiveExample` action args (editor/🟦️.ts:1172–1183) as `ActionArgOption` entries only
- **Not** in `manifest.examples` because `EditorBuilder` offers no API to register them
- Comment (lines 1218–1224) explicitly marks this as an SDK gap

**Required change** (minimum literal):

In `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs`, add to `AppBuilder`:

```rust
pub async fn example(mut self, source: impl Into<ExampleSource>) -> Self {
    // Store in a pending examples vec; flush on .build_definition()
    self.pending_examples.push(ExampleDefinition::from(source.into()));
    self
}
```

Then in `from_builder()` / `try_from_builder()`: return examples from that vec, not empty.

Then call in procedural/🦀️.rs line 312:
```rust
.editor_examples([
  crate::editor::generation3d::examples::hexagonal_mushroom_column::source(),
  // ... 7 more
])
```
