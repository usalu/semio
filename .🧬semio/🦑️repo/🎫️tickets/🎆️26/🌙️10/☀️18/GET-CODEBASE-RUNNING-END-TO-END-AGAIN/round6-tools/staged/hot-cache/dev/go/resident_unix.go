//go:build !windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🛰️resident-unix dials the service over a unix socket and refuses a runtime directory that is not private and owned by the caller.

// #endregion 🧲️Header

package launcher

import (
	"io"
	"net"
	"os"
	"syscall"
	"time"
)

// #region 🛰️ResidentUnix

const residentGOOS = "unix"

func residentDialEndpoint(endpoint string) (io.ReadWriteCloser, error) {
	return net.DialTimeout("unix", endpoint, 250*time.Millisecond)
}

func residentTrusted(directory string) bool {
	info, err := os.Lstat(directory)
	if err != nil || !info.IsDir() {
		return false
	}
	status, ok := info.Sys().(*syscall.Stat_t)
	return ok && int(status.Uid) == os.Getuid() && info.Mode().Perm()&0o077 == 0
}

// #endregion 🛰️ResidentUnix
