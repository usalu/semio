// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package core

// #endregion Package

// #region Types
type ErrorCode string

const (
	ErrInternal ErrorCode = "E_INTERNAL"
	ErrParse    ErrorCode = "E_PARSE"
	ErrCanceled ErrorCode = "E_CANCELED"
	ErrNetwork  ErrorCode = "E_NETWORK"
	ErrAuth     ErrorCode = "E_AUTH"
)

// #endregion Types
