#!/bin/sh
# usage: s17-final.sh  (slice 17: the three registered real-world routes in turn, then the descriptor regeneration; logs s17f-* in 🗑️generated)
cd "$(dirname "$0")"
sh ./retry-route.sh s17f-site registered_real_world_site_plan 18
sh ./retry-route.sh s17f-thesis registered_bachelor_thesis 18
sh ./retry-route.sh s17f-inst registered_real_world_institution 18
sh ./s17-descriptor.sh
