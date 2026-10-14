# Plugin Production Build Diagnostics

The focused production integration target excludes unrelated lib-test fixtures while linking the real library. Its actual prototype build failed with the following complete 45 current production errors. No Entry/output error was reported in this diagnostic census. The earlier whole lib-test attempt reported 606 errors; its truncated failure excerpt did not show the complete scope. Full library and native output runtime qualification remain pending.

| Code | Canonical Source | Line | Error |
| --- | --- | --- | --- |
| E0433 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/🎟️issuer/🦀️.rs | 25 | cannot find `typed_operation` in `tool_jobs` |
| E0433 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/🎟️issuer/🦀️.rs | 26 | cannot find `typed_operation` in `tool_jobs` |
| E0433 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/🎟️issuer/🦀️.rs | 30 | cannot find `typed_operation` in `tool_jobs` |
| E0425 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs | 146 | cannot find value `tree` in this scope |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs | 2172 | this method takes 7 arguments but 6 arguments were supplied |
| E0609 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs | 2889 | no field `root` on type `()` |
| E0609 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs | 2895 | no field `root` on type `()` |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs | 4165 | `if` and `else` have incompatible types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs | 4185 | this method takes 7 arguments but 6 arguments were supplied |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs | 4185 | mismatched types |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs | 4265 | mismatched types |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs | 4265 | mismatched types |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1019 | arguments to this function are incorrect |
| E0609 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1020 | no field `frames` on type `Result<std::option::Option<PluginExchangeOutput>, semio_framework_job::OriginalTurnFailure>` |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1020 | the size for values of type `[u8]` cannot be known at compilation time |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1020 | the size for values of type `[u8]` cannot be known at compilation time |
| E0609 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1023 | no field `effects` on type `Result<std::option::Option<PluginExchangeOutput>, semio_framework_job::OriginalTurnFailure>` |
| E0609 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1028 | no field `events` on type `Result<std::option::Option<PluginExchangeOutput>, semio_framework_job::OriginalTurnFailure>` |
| E0609 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1441 | no field `frames` on type `std::option::Option<PluginExchangeOutput>` |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1441 | the size for values of type `[u8]` cannot be known at compilation time |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1441 | the size for values of type `[u8]` cannot be known at compilation time |
| E0609 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1444 | no field `effects` on type `std::option::Option<PluginExchangeOutput>` |
| E0609 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1449 | no field `events` on type `std::option::Option<PluginExchangeOutput>` |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs | 1455 | mismatched types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1255 | this method takes 2 arguments but 1 argument was supplied |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1255 | `?` operator has incompatible types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 5948 | this method takes 2 arguments but 1 argument was supplied |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 5969 | mismatched types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 6302 | this method takes 2 arguments but 1 argument was supplied |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 6302 | `?` couldn't convert the error to `semio_framework_dsl::Fault` |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 6303 | mismatched types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 7053 | this method takes 2 arguments but 1 argument was supplied |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 7111 | mismatched types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 8531 | this method takes 2 arguments but 1 argument was supplied |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 8531 | `?` couldn't convert the error to `semio_framework_dsl::Fault` |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 8543 | mismatched types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 8667 | this method takes 2 arguments but 1 argument was supplied |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 8667 | `?` couldn't convert the error to `semio_framework_dsl::Fault` |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 8667 | `match` arms have incompatible types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🤝️group/🦀️.rs | 25 | this method takes 2 arguments but 1 argument was supplied |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🤝️group/🦀️.rs | 25 | `?` couldn't convert the error to `semio_framework_dsl::Fault` |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🤝️group/🦀️.rs | 25 | `match` arms have incompatible types |
| E0061 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1157 | this method takes 2 arguments but 1 argument was supplied |
| E0277 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1157 | `?` couldn't convert the error to `semio_framework_dsl::Fault` |
| E0308 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1159 | mismatched types |
