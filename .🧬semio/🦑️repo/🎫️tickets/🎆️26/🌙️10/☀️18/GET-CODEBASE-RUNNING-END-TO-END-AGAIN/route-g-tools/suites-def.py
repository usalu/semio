"""Suite definitions: suite id -> trees (relative to the build module root). Longest tree wins, like the runtime router."""
S = {
 # command surface
 "build-command-line": ["⌨️command","📤️arguments","⚙️operations","🚪️door","🚀️bootstrap/⌨️cli","🚀️bootstrap/🧪️tests","🧭️invocation","🟢️readiness","⏱️profile","📖️texts"],
 "build-parsers": ["🚀️bootstrap/📦️parsers"],
 "build-tool-setup": ["🧰️tool-setup","🧰️tool-registry","🛠️cmake","🟦️typescript","🔌️backends"],
 "build-doctor-preflight": ["🩺️doctor","🩺️preflight","🧾️ci-report","🪪️agent-sessions","📐️conformance","🪞️parity","🧾️store-receipt"],
 "build-targets-selection": ["🎯️targets","🧭️selection","🎚️mode","🧩️units","🧷️format-stamps"],
 # resident daemon, workers
 "build-resident": ["🏡️resident"],
 "build-workers": ["👷️workers","🧵️workers"],
 # leases / capacity / memory
 "build-leases-command": ["🔒️leases"],
 "build-host-capacity": ["🌡️host-capacity"],
 "build-memory": ["🧠️memory"],
 "build-caller-birth": ["📥️caller"],
 # observation / incremental
 "build-observation": ["👁️observation"],
 "build-incremental": ["📏️incremental"],
 "build-incremental-bench": ["📏️incremental/🏎️bench"],
 "build-incremental-concurrent": ["📏️incremental/🤝️concurrent"],
 # remote
 "build-remote-cache": ["🌐️remote-cache"],
 "build-remote-execution": ["🌐️remote-execution"],
 # execution
 "build-execution": ["🏃️execution"],
 "build-cache-gc": ["🧹️cache-gc"],
 "build-reporting": ["📊️reporting"],
 "build-storybook": ["📖️storybook"],
 # inputs
 "build-inputs-storage": ["📥️inputs/💾️storage"],
 "build-inputs-selected": ["📥️inputs/🎯️selected","📥️inputs/🎯️precision","📥️inputs/📋️data","📥️inputs/📂️membership"],
 "build-inputs-revision": ["📥️inputs","📥️inputs/🔏️revision","📥️inputs/🦀️resolution","📥️inputs/🧊️stable-file","📥️inputs/🧮️digest","📥️inputs/🪶️lean-inventory","📥️inputs/🧪️tests"],
 "build-source-closure": ["📥️source"],
 # tools: bazel
 "build-bazel-sessions": ["🔧️tools/🏗️bazel/🧵️sessions"],
 "build-bazel-execution": ["🔧️tools/🏗️bazel/🏃️execution","🔧️tools/🏗️bazel/🎬️executor","🔧️tools/🏗️bazel/🎬️scopes","🔧️tools/🏗️bazel/🎯️affected","🔧️tools/🏗️bazel/⚙️operations","🔧️tools/🏗️bazel/🧭️commands"],
 "build-bazel-evidence": ["🔧️tools/🏗️bazel/👁️evidence","🔧️tools/🏗️bazel/🚦️test-classes","🔧️tools/🏗️bazel/🧾️test-results","🔧️tools/🏗️bazel/🪵️failure-tail"],
 "build-bazel-snapshots": ["🔧️tools/🏗️bazel/📐️path-plan","🔧️tools/🏗️bazel/📸️snapshot","🔧️tools/🏗️bazel/🔁️probe"],
 "build-bazel-host": ["🔧️tools/🏗️bazel/🌡️host-capacity","🔧️tools/🏗️bazel/💾️storage","🔧️tools/🏗️bazel/👷️workers","🔧️tools/🏗️bazel/🧰️toolchains","🔧️tools/🏗️bazel/🐹️go","🔧️tools/🏗️bazel/🦀️cargo"],
 "build-bazel-contracts": ["🔧️tools/🏗️bazel","🔧️tools/🏗️bazel/🧪️tests"],
 "build-nx-tool": ["🔧️tools","🔧️tools/🧶️nx","🔧️tools/🐙️semio","🔧️tools/💾️storage"],
 # graph
 "build-generation-claims": ["🕸️graph/🕰️generations/🧩️claims"],
 "build-generation-pin": ["🕸️graph/🕰️generations/🧷️pin"],
 "build-generation-coverage": ["🕸️graph/🕰️generations/🔭️coverage"],
 "build-generation-facts": ["🕸️graph/🕰️generations/🧮️fact-codec","🕸️graph/🕰️generations/🧪️tests/🧮️fact-index"],
 "build-generation-load": ["🕸️graph/🕰️generations/🔌️load","🕸️graph/🕰️generations/🌊️json-stream","🕸️graph/🕰️generations/🌍️environment"],
 "build-generations": ["🕸️graph/🕰️generations"],
 "build-graph-protocol": ["🕸️graph","🕸️graph/🧪️tests","🕸️graph/🗂️generation-files","🕸️graph/🚪️admission","🕸️graph/⏱️phases","🕸️graph/🔁retry"],
 "build-graph-owners": ["🕸️graph/🧭️file-owner","🕸️graph/🧾️collect-errors","🕸️graph/🧾️registry-consistency"],
 "build-graph-rust": ["🕸️graph/🦀️rust"],
 "build-inference": ["🕸️graph/📥️inference","🕸️graph/📥️inference/🧪️tests"],
 "build-inference-projects": ["🕸️graph/📥️inference/📦️project-nodes","🕸️graph/📥️inference/🔎️owner-discovery","🕸️graph/📥️inference/🔖️stamps"],
 "build-inference-policy": ["🕸️graph/📥️inference/🎯️target-policy","🕸️graph/📥️inference/🏭️generator-outputs","🕸️graph/📥️inference/🕸️dependency-edges","🕸️graph/📥️inference/🦀️native-manifests"],
 "build-inference-script-inputs": ["🕸️graph/📥️inference/🧭️script-inputs","🕸️graph/📥️inference/🪢️revision-graph"],
 # build root laws
 "build-workspace-graph-laws": ["🧪️tests/🕸️graph"],
 "build-workspace-phase-laws": ["🧪️tests/🎟️phase","🧪️tests/🎯️source-roster","🧪️tests/🏗️native"],
 "build-workspace-laws": ["🧪️tests"],
 # self
 "build-route-table": ["🗂️suites"],
}
