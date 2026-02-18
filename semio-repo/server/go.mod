module github.com/usalu/semio/semio-repo/server

go 1.24.0

require (
	github.com/usalu/semio/semio-repo/go v0.0.0-00010101000000-000000000000
	modernc.org/sqlite v1.45.0
)

replace github.com/usalu/semio/semio-repo/go => ../go

require (
	github.com/dustin/go-humanize v1.0.1 // indirect
	github.com/google/uuid v1.6.0 // indirect
	github.com/mattn/go-isatty v0.0.20 // indirect
	github.com/ncruces/go-strftime v1.0.0 // indirect
	github.com/remyoudompheng/bigfft v0.0.0-20230129092748-24d4a6f8daec // indirect
	golang.org/x/exp v0.0.0-20260112195511-716be5621a96 // indirect
	golang.org/x/sys v0.40.0 // indirect
	modernc.org/libc v1.67.7 // indirect
	modernc.org/mathutil v1.7.1 // indirect
	modernc.org/memory v1.11.0 // indirect
)
