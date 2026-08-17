# CW2 finding — why the thread_local child caches cannot simply be deleted

I attempted the writer cache-kill (the plan's CW3 pilot) and stopped deliberately after
establishing that it is blocked on a **design decision, not a mechanical migration**. Recording the
real question so the next session does not rediscover it.

## The seam exists; the read sites cannot reach it

CW1 landed `ArtifactView.children` (`ChildContentView`), which resolves a child's live content
through its real store and cannot go stale. Writer's own cache doc comment names precisely this as
the missing piece:

> Exists because writer edits at keystroke granularity and no `LinkResolver`/child-dispatch seam is
> wired into `ArtifactApp::handle` yet … until one exists, the only way a persisted
> content-addressed HANDLE can round-trip to real text within one process is this cache.

So the stated blocker is gone. But the cache has **53 read sites across 18 files**, and they are not
all in `handle`/`render`:

| Read site class | Has an `ArtifactView`? |
|---|---|
| app commands, panels, window render | ✅ yes — trivially migratable |
| `💡️inferences` | ⚠️ only if inference gains a child-resolution context |
| `🚪️io` serializers (pdf, docx) | ❌ no |
| `📸️snapshot/📝️text` DSL printer/parser | ❌ **no, and structurally cannot** |
| `🔺️diff/📝️text`, `🧬️mutations/💾️binary`, `↩️inverse` | ❌ **no, and structurally cannot** |

## The structural blocker

```rust
fn print_dsl(&self) -> String;              // 🏪️store/🦀️component.rs:890
fn encode_pack_with(&self, options) -> …;   // ArtifactPack
```

Neither takes any context. A codec receives **only** `&Snapshot`. It therefore cannot reach a child
store, a `LinkResolver`, or a `ChildContentView` — by construction, not by omission. This is why the
cache exists, and no amount of threading `ArtifactView` through app code removes the need for it as
long as the codec is expected to emit resolved child TEXT.

## The actual question to decide

**Should a parent's serialized form contain its child's resolved content, or only the handle?**

UCAS's own design says the parent holds two strings and *"a parent's diff NEVER embeds a child
diff"*. Taken consistently, that implies:

- `📸️snapshot/📝️text` should print the **handle**, not the resolved text.
- `🔺️diff` / `↩️inverse` / `💾️binary` should likewise operate on handles only.
- Resolution happens **only** where a view exists: render, export, inference — i.e. exactly the
  places that legitimately have a `ChildContentView`.
- The child's own text is persisted by the child's own envelope, which CW1 made real
  (`LoadChildren`/`ReadChildren`, and the `REC_COMPOSITION` overlay that lets a reopened child know
  what it is).

If that is the intended reading, the migration is: **change what the codecs emit**, then delete the
cache — and every shipped writer fixture must be regenerated, because the on-disk DSL changes shape.
That is a real wave with fixture churn, not a refactor.

The alternative (give codecs a resolution context) would mean changing `ArtifactDsl`/`ArtifactPack`
signatures repo-wide — every artifact, composing or not — to carry a context that only a handful of
composing artifacts would ever use.

## Why I stopped rather than proceeding

Doing half of this — migrating the app-level read sites while the codecs still expect resolved text
— leaves the cache in place AND adds a second read path, i.e. strictly worse than today. And getting
the codec change wrong is exactly the silent-data-loss class that already bit cad once in this
ticket family (fields present on the struct, wired into neither codec, `cargo check` green
throughout).

## Recommended next step

Decide the question above (I believe handle-only is correct and consistent with UCAS's stated
design), then execute per plugin: codecs first, fixtures regenerated, app read sites moved to
`ArtifactView.children`, cache deleted, plugin suite compared against its recorded baseline.

Affected plugins carrying a child-content cache: `✒️writer`, `💠️lowpoly`, `📐️cad`, `🌊️flow`,
`🏭️process`, `🕸️dag`, `🎬️sequence`, `📜️imperative`, `➗️mathematical`, `🪐️space`.
The remaining `thread_local!` sites in the plugin tree (`🖍️draw`, `🧩️puzzle`, `🎥️shooting`,
`📋️forms`, `📏️layout`, `🖨️raster`, `📖️playbook`, `💡️reasoning`, `🎞️animate`, `📕️norm`) need
triage first — some are legitimate ephemeral engine state under the DKM `EngineRep` doctrine, not
child caches, and those are governed by the transient lane / `EngineRep` rules instead.
