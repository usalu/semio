//go:build windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛰️resident-windows opens the named pipe of the service as a file; the pipe is created by the same user and the access control of the pipe is the boundary.

// #endregion 🧲️Header

package launcher

import (
	"io"
	"os"
)

// #region 🛰️ResidentWindows

const residentGOOS = "windows"

func residentDialEndpoint(endpoint string) (io.ReadWriteCloser, error) {
	return os.OpenFile(endpoint, os.O_RDWR, 0)
}

func residentTrusted(string) bool { return true }

// #endregion 🛰️ResidentWindows
