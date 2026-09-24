#!/bin/zsh
# 🌎️ W1: full catalog of every package whose editor kinds all answer `pack-schema-hash` today (report §4.2);
# extended as slice p4 restores the derive path on the other 18.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
W1_PACKAGES=stdio,gis,note,draw,wfc,vcs,flow,remodel,space,sourcing,shooting,fem,block,architect,demonstrator,procedural W1_DATA_NAME=w1-catalog zsh w1-catalog.sh publish
