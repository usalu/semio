// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package cli

// #endregion Package

// #region Imports
import "fmt"

// #endregion Imports

// #region Types
type ExitError struct {
	Code int
	Err  error
}

// #endregion Types

// #region Errors
func (e ExitError) Error() string {
	if e.Err != nil {
		return e.Err.Error()
	}
	return fmt.Sprintf("exit code %d", e.Code)
}

func (e ExitError) Unwrap() error {
	return e.Err
}

// #endregion Errors
