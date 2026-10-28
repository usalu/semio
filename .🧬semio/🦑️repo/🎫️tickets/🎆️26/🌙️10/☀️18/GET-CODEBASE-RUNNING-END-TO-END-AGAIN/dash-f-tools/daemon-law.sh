#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
unset SEMIO_SCRIPT_STALL_MS
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test -p semio-framework-repo-dashboard-daemon -- --test-threads 4 daemons_that_start_at_the_same_moment the_waiting_line_is_written_only process_deadline_stops_external_descendants a_daemon_socket_path
echo "rc=$?"
