// #region 🧲Header
// Copilot-native MCP + hook entry (stdio MCP or `hook <event>` with JSON on stdin).
// #endregion 🧲Header

package main

import (
	"errors"
	"fmt"
	"io"
	"os"

	"github.com/usalu/semio/repo/client"
)

func main() {
	if len(os.Args) >= 2 && os.Args[1] == "hook" {
		if len(os.Args) < 3 {
			fmt.Fprintln(os.Stderr, "usage: hook <event>")
			os.Exit(2)
		}
		stdin, _ := io.ReadAll(os.Stdin)
		if err := client.RunHookFor(client.McpClientCopilot, os.Args[2], stdin); err != nil {
			var ex client.ExitError
			if errors.As(err, &ex) {
				os.Exit(ex.Code)
			}
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
		return
	}
	if err := client.RunMCPFor(client.McpClientCopilot); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
