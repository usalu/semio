#!/bin/sh
# 🏊️ After GO and `bun repo tools setup nx`: the numbers the farm is accepted by. Run them on a quiet host (load below 12), one command at a time, niced.
#   cold, farm:      the first `SEMIO_GRAPH_PHASES=1 bun repo show projects` after the apply is cold by itself (the implementation revision changed);
#                    it prints the farm lines ("graph: inferring N projects with W worker processes"), the [graph-phase] rows and one [graph-farm] statistics line.   target <= 15 min
#   warm:            run the same command again: no worker starts, nothing is asked of the host capacity gate.                                                             target <= 30 s
#   cold, serial:    SEMIO_GRAPH_WORKERS=0 bun repo show projects   after the next implementation change (or in a second checkout): the reference time and the reference graph.
#   knobs:           SEMIO_GRAPH_WORKERS=0|auto|N   SEMIO_GRAPH_WORKER_MB=<512 or more>   SEMIO_GRAPH_PHASES=1
# Compare the two graphs with the tool the coordinator uses for graph identity (conc-graph's measure-after-go.sh --verify runs the unchanged full inference as the oracle).
echo "see the comments of this file"
