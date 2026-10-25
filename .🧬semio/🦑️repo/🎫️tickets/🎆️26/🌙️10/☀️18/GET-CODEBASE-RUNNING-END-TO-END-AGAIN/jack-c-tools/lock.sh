#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
for scope in "♻️mit-bestand" "🏢️semio-tech" "🌎️hub" "✏️s"; do
  echo "== lock $scope"
  nice -n 10 bun repo dependencies lock --scope "$scope" || echo "== FAILED $scope"
done
echo "== done"
