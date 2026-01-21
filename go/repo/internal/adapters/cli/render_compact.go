// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package cli

// #endregion Package

// #region Imports
import (
	"encoding/json"
	"io"

	"github.com/usalu/semio/go/repo/internal/events"
)

// #endregion Imports

// #region Renderers
func RenderCompact(out io.Writer, errOut io.Writer, stream <-chan events.Event, verbose bool) (int, error) {
	exitCode := 0
	for event := range stream {
		if event.Kind == events.KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
			continue
		}
		if event.Kind == events.KindError && event.Error != nil {
			if event.Error.Detail != "" && verbose {
				if _, err := errOut.Write([]byte(event.Error.Detail + "\n")); err != nil {
					return exitCode, err
				}
			}
			if event.Error.Message != "" {
				if _, err := errOut.Write([]byte(event.Error.Message + "\n")); err != nil {
					return exitCode, err
				}
			}
			continue
		}
		if event.Kind == events.KindLog && event.Message != "" {
			if _, err := errOut.Write([]byte(event.Message + "\n")); err != nil {
				return exitCode, err
			}
			continue
		}
		if event.Kind == events.KindResult && len(event.Data) > 0 {
			formatted := event.Data
			var decoded interface{}
			if err := json.Unmarshal(event.Data, &decoded); err == nil {
				if pretty, err := json.MarshalIndent(decoded, "", "  "); err == nil {
					formatted = pretty
				}
			}
			if _, err := out.Write(append(formatted, '\n')); err != nil {
				return exitCode, err
			}
		}
	}
	return exitCode, nil
}

// #endregion Renderers
