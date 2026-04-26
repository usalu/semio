// #region 🧲Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

// Repo MCP entry point.

// #endregion 🧲Header

// #region 🤸Preamble
// Package declaration and dependency imports for the repo MCP entry point.

package main

import (
	"fmt"
	"os"

	"github.com/usalu/semio/repo/client"
)

// #endregion 🤸Preamble

// #region 🦀Mcp
// MCP process bootstrap.

func main() {
	if err := client.RunMCP(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

// #endregion 🦀Mcp
