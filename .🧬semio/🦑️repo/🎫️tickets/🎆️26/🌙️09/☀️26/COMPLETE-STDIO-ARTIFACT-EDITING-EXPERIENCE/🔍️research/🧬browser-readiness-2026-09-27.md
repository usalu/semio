# Browser Readiness

The live local server at `http://127.0.0.1:6013/` was inspected through the supported browser API. It presents the puzzle 3D playground with the Concrete Forest example. Its console reports stale/unactivated staged components, including stdio. It is not evidence for this task’s current artifact editing changes. The temporary background inspection tab was closed. The intended stdio CSV preview on port 6212 remains stopped. A fresh build/activation and an editor-specific launch are required before browser acceptance.

## Fresh CSV Preview Attempt

After confirming port 6212 had no listener, root restarted the existing launch.json CSV React command through `workspace:dev`, with its authored port/plugin/app environment. Log: `🗑️generated/stdio-csv-preview-current-2.log`. This begins build/staging only; it is not browser acceptance. The unrelated puzzle preview on port 6013 remains untouched. Full optimized component current11 is separately running after the DOCX borrowed canonical-value helper lifetime repair.

CSV preview attempt current2 failed before compilation: the nested workspace bootstrap required an Nx daemon which did not start. Its outer Nx footer misleadingly printed success despite the child error; no server or browser pass is claimed. Root then invoked the same registered `@semio-tech/framework-os-dev:dev` Nx target directly with the unchanged CSV launch environment and daemon disabled (current3 log), preserving the public script/task implementation and avoiding daemon resets that could affect concurrent work.

## Actual CSV Browser Acceptance Started

Concrete `dev-stdio-csv-react-dev` current4 completed all14 dependencies, activated the component and served6212. Initial browser navigation timed out while the module graph loaded, then the actual CSV Demo appeared. Current runtime acceptance and the observed failed Redo/empty Details are recorded in `🧬details-first-paint-and-redo-2026-09-27.md`. Thus the earlier stopped/no-browser state above is superseded, but acceptance is not green.
