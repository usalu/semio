# Current Authored Linux Gates

Read directly from the current authored workspace project after the separate MCP sealed-graph query observation timed out. These are declared entry points and dependencies, not execution results or proof of a current sealed graph. Existing actual gates/owners remain authoritative in linux-current-acceptance.md.

| Target | Command | Dependencies |
|---|---|---|
| workspace:setup | repo ./📜️script.ts setup | ["deps-build-tools","deps-js-all","deps-python","deps-cargo","deps-go","deps-dotnet","deps-cpp","deps-browsers","deps-wasm","deps-tools","setup-git","prepare","repo-mcp:install","@semio-tech/framework-os-mcp-rs:build"] |
| workspace:deps-build-tools | repo ./📜️script.ts repo tools setup all | [] |
| workspace:setup-native | repo ./📜️script.ts setup native | [] |
| workspace:setup-devcontainer-start | repo ./📜️script.ts setup devcontainer start | [] |
| workspace:setup-devcontainer-attach | repo ./📜️script.ts setup devcontainer attach | [] |
| workspace:build | repo:noop | [{"target":"build","projects":["*","!workspace"]},"build-storybook"] |
| workspace:test | repo ./📜️script.ts test | [{"target":"test","projects":["*","!workspace","!@semio-tech/repo-test-domain"],"params":"forward"},"@semio-tech/repo-test-domain:test-contract"] |
| workspace:lint | repo ./📜️script.ts lint | [{"target":"lint","projects":["*","!workspace"]}] |
| workspace:verify-typescript | repo ./📜️script.ts verify typescript check | [] |
| workspace:test-contract | repo ./📜️script.ts test contract | [] |
| workspace:test-oracle | repo ./📜️script.ts test oracle | [] |
| workspace:test-subject | repo ./📜️script.ts test subject | [] |
| workspace:test-parity | repo ./📜️script.ts test parity | [] |
| workspace:cpp-setup | repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts cpp setup | [] |
| workspace:cpp-configure | repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts cpp configure | ["cpp-setup"] |
| workspace:cpp-build | repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts cpp build | [{"target":"cpp-configure","params":"forward"}] |
| workspace:cpp-test | repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts cpp test | [{"target":"cpp-build","params":"forward"}] |
| workspace:cpp-test-toolchain | repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts cpp test-toolchain | [] |
