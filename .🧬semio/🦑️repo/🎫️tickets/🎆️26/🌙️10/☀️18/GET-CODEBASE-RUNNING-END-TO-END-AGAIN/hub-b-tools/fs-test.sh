#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test -p semio-framework-server -p semio-framework-server-tests -p semio-framework-server-contract -p semio-framework-server-storage -p semio-framework-server-policy -p semio-framework-server-authority -p semio-framework-server-throttle || exit $?
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-server --features conformance
