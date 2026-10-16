import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import barity
new_inf, ext, snap, live = sys.argv[1:5]
barity.compare(new_inf, ext, os.path.join(snap, '🎲️board'), live)
