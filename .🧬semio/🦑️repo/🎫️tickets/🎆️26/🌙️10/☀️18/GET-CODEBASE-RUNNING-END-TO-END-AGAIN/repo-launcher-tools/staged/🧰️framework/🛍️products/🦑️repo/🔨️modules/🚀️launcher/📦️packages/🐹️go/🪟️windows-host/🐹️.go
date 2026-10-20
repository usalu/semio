//go:build windows

// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🪟️windows-host runs the runtime as a child that shares the console, forwards interrupts and settles with its exit code, and reads the user locale.

// #endregion 🧲️Header

package launcher

import (
	"errors"
	"os"
	"os/exec"
	"os/signal"
	"syscall"
	"unsafe"
)

// #region 🪟️Host

// 🔄️handOff starts the runtime with the inherited streams and waits for its terminal result; a non-zero exit code is returned as ExitError.
func handOff(executable string, argv []string, env []string) error {
	command := exec.Command(executable, argv[1:]...)
	command.Stdin, command.Stdout, command.Stderr = os.Stdin, os.Stdout, os.Stderr
	command.Env = env
	interrupts := make(chan os.Signal, 2)
	signal.Notify(interrupts, os.Interrupt)
	defer signal.Stop(interrupts)
	if err := command.Start(); err != nil {
		return err
	}
	done := make(chan struct{})
	defer close(done)
	go func() {
		for {
			select {
			case received := <-interrupts:
				_ = command.Process.Signal(received)
			case <-done:
				return
			}
		}
	}()
	err := command.Wait()
	var exited *exec.ExitError
	if errors.As(err, &exited) {
		code := exited.ExitCode()
		if code < 0 {
			code = 130
		}
		return ExitError{Code: code}
	}
	return err
}

// 🖥️systemLanguages lists the user locale name of Windows.
func systemLanguages() []string {
	buffer := make([]uint16, 85)
	found, _, _ := syscall.NewLazyDLL("kernel32.dll").NewProc("GetUserDefaultLocaleName").Call(uintptr(unsafe.Pointer(&buffer[0])), uintptr(len(buffer)))
	if found == 0 {
		return nil
	}
	return []string{syscall.UTF16ToString(buffer)}
}

// #endregion 🪟️Host
