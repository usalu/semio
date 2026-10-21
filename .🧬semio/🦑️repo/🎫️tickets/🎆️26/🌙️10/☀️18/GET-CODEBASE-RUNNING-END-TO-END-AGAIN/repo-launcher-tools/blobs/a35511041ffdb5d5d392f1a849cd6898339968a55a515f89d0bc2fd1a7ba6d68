//go:build !windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🐚️unix-host hands the process over to the runtime in place, so the PID, streams, signals and exit code are the runtime's own, and reads the system language list.

// #endregion 🧲️Header

package launcher

import (
	"context"
	"os/exec"
	"os/signal"
	"regexp"
	"runtime"
	"syscall"
	"time"
)

// #region 🐚️Host

var appleLanguage = regexp.MustCompile(`"?([A-Za-z]{2,3}(?:-[A-Za-z0-9]+)*)"?\s*[,)]`)

// 🔄️handOff replaces this process with the runtime; it returns only when the replacement failed.
func handOff(executable string, argv []string, env []string) error {
	signal.Reset()
	return syscall.Exec(executable, argv, env)
}

// 🖥️systemLanguages lists the macOS preferred languages; other Unix systems state theirs in the environment.
func systemLanguages() []string {
	if runtime.GOOS != "darwin" {
		return nil
	}
	deadline, stop := context.WithTimeout(context.Background(), 3*time.Second)
	defer stop()
	output, err := exec.CommandContext(deadline, "defaults", "read", "-g", "AppleLanguages").Output()
	if err != nil {
		return nil
	}
	var tags []string
	for _, match := range appleLanguage.FindAllStringSubmatch(string(output), -1) {
		tags = append(tags, match[1])
	}
	return tags
}

// #endregion 🐚️Host
