// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🚀️launcher starts the repository implementation through a JavaScript runtime that it provisions itself: it resolves the repository root, pins the runtime from the schema-declared registry, and hands the process over with the exact arguments.

// #endregion 🧲️Header

package launcher

import "fmt"

// #region 🚀️Contract

// 🔖️Version is the version of this launcher.
const Version = "1.0.0"

// 📜️Contract is the runtime registry contract this launcher implements.
const Contract = 1

// 🗂️RegistryPath is the repository-relative path of the runtime registry, the only path the launcher knows by heart.
const RegistryPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json"

// 🌍️Environment reads one environment variable and answers an empty string when it is unset.
type Environment func(name string) string

// 🚦️ExitError carries the exit code of the started implementation.
type ExitError struct{ Code int }

// 🔤️Error names the exit code.
func (e ExitError) Error() string { return fmt.Sprintf("exit status %d", e.Code) }

// #endregion 🚀️Contract
