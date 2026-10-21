// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 📦️main is the stand-alone `repo` launcher executable.

// #endregion 🧲️Header

package main

import (
	"context"
	"os"
	"os/signal"
	"syscall"

	launcher "github.com/usalu/semio/repo/launcher"
)

func main() {
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	code := launcher.DefaultHost().Main(ctx, os.Args[1:])
	stop()
	os.Exit(code)
}
