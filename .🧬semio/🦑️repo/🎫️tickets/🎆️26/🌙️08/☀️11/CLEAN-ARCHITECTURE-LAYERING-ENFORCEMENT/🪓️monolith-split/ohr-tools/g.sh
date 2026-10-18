#!/bin/zsh
exec git -c core.fsmonitor=false -c core.quotepath=false "$@"
