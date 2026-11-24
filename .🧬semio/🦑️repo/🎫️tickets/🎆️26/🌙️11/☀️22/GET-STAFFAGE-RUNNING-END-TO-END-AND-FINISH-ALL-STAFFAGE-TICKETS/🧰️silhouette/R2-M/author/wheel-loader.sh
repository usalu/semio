#!/bin/zsh
# 🚧️ Repair ops of `wheel-loader` of the second round (after the ops of SAM): the hand repairs (wheel-loader.py), then at 1:50 the four tyres in front elevation and plan as carcass and lugs project together (the drawn tyre was the smooth envelope, 30 mm fuller at the shoulders than the lugged body), the cab roof in the plan of 1:50 as its loft projects (the drawn outline was up to 2 mm fuller at the corners, which left a sliver beside the outer work lamps) with the two front steering lugs whole beside it, and at 1:200 the two hinge plates of the bucket in the front elevation.
source "${0:A:h}/../env.sh"; cd "$W/author" && python3 wheel-loader.py && cd "$W"
for w in f-l f-r r-l r-r; do for v in front plan; do
  "$B" trace.ts wheel-loader 50 $v "tyre-$w,lugs-$w-*" --id tyre-$w --bone wheel-${w%-?} --like tyre-$w --replace tyre-$w | tail -1
done; done
"$B" trace.ts wheel-loader 50 plan cab-roof --id cab-roof --bone rear-frame --like cab-roof --replace cab-roof | tail -1
for side in l r; do "$B" trace.ts wheel-loader 50 plan steer-lug-front-$side --id steer-lug-front-$side --bone front-frame --like steer-lug-front-$side --replace steer-lug-front-$side | tail -1; done
"$B" trace.ts wheel-loader 200 front bucket-hinge --id bucket-hinge --bone bucket --like bucket --z 1.45 --tone 0.2 --after bucket | tail -1
"$B" drawable.ts wheel-loader --ops ops/wheel-loader.json
