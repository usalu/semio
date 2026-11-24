#!/bin/zsh
# ✂️ Crops a sheet rendered by look.sh to a part given in fractions of its size and prints the path: crop.sh <png> <x0> <y0> <x1> <y1> [max size]
P="$1"; W0=$(sips -g pixelWidth "$P" | awk '/pixelWidth/{print $2}'); H0=$(sips -g pixelHeight "$P" | awk '/pixelHeight/{print $2}')
X=$(printf "%.0f" $(( W0 * $2 ))); Y=$(printf "%.0f" $(( H0 * $3 ))); CW=$(printf "%.0f" $(( W0 * ($4 - $2) ))); CH=$(printf "%.0f" $(( H0 * ($5 - $3) )))
O="${P:r}.crop-$2-$3.png"
sips -c $CH $CW --cropOffset $Y $X "$P" --out "$O" > /dev/null && sips -Z ${6:-900} "$O" > /dev/null; echo "$O"
