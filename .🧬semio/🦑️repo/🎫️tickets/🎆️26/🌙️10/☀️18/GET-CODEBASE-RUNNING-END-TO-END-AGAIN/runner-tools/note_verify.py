"""📝 Records the workspace:verify verdict (one-shot ticket helper)."""
import os
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
t = t.rstrip("\n") + "\n" + ("| rn-verify | workspace:verify | 01:29 | 126 s | 4,121 MB | 1 | RED before Bazel: inferring the `workspace` project refuses `build.graph.rust-sources-unresolved` for `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust` (reason `build-script`: "
       "`📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs:89/118` `cargoSourcesOrRefuseV1` refuses every crate with a `build.rs`; this one is the ASCII-named adapter that `include!`s `🏗️builder/🦀️.rs`). That module carries UNSTAGED live edits now; no batch |\n")
open(p, "w", encoding="utf8").write(t)
