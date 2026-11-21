# Course Assignment PDF Receipt Audit

The course manifest declares six published assignments. Parametrization, typology, and heating and cooling each publish one English and one German PDF. Their producers are the corresponding `build-…-en` and `build-…-de` targets in the architecture course project.

The mounted products partition `.cache/b/p/m` was readable. It held no `print` directory, so all six assignment publications and the mode-specific print font publication were absent in both prototype and release. The tools partition contained the Tectonic executable and its receipts under `print/tectonic/aarch64-apple-darwin`. The TeX bundle layout `print/tex` was absent.

The course declares four prerequisites for each print target. The compiler retains and verifies current Tectonic, TeX bundle and font receipts, and `print:generate` prepares the shared token stylesheet. The missing bundle and fonts prevent running any of the six PDF targets with `--exclude-dependencies`.

Launching the declared font subcommand directly through `repo` exited at startup with `Original parent Script process invocation required`. The Print storage operation requires the authentic native or build orchestration caller and uses its build mode. The canonical course gate already schedules all six PDF producers and their prerequisites, so this audit left that build queue untouched.

The canonical build published all six prototype documents and their receipts. The canonical development site delivered all six as application/pdf with HTTP 200. Each response had a PDF signature and matched the published artifact by SHA-256.
