. "$(dirname "${BASH_SOURCE[0]}")/humansb-C2-env.sh"
. "$G/pp.sh"
export TY0=0.02 TY1=0.07 THR=0.009
export GD="$(python3 "$N/humansb-C2-stadium.py" 0.718 1.058 0.8 1.05 0.018 8)"
