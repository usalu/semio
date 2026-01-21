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
func RenderJSONL(out io.Writer, stream <-chan events.Event) (int, error) {
	encoder := json.NewEncoder(out)
	encoder.SetEscapeHTML(false)
	exitCode := 0
	for event := range stream {
		if event.Kind == events.KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
		}
		if err := encoder.Encode(event); err != nil {
			return exitCode, err
		}
	}
	return exitCode, nil
}

// #endregion Renderers
