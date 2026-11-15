import sys
from pathlib import Path

path = Path(sys.argv[1])
s = path.read_text(encoding="utf-8")
pairs = [
 ('         r"$154\,\mathrm{kWh/(m^2\,a)}$ against $56\,\mathrm{kWh/(m^2\,a)}$ of primary energy $Q_{P}$.",\n         r"$154\,\mathrm{kWh/(m^2\,a)}$ gegen $56\,\mathrm{kWh/(m^2\,a)}$ Primärenergie $Q_{P}$."),',
  '         rf"${Q_PRIM_GAS}\,\mathrm{{kWh/(m^2\,a)}}$ against ${Q_PRIM_WP}\,\mathrm{{kWh/(m^2\,a)}}$ of primary energy $Q_{{P}}$.",\n         rf"${Q_PRIM_GAS}\,\mathrm{{kWh/(m^2\,a)}}$ gegen ${Q_PRIM_WP}\,\mathrm{{kWh/(m^2\,a)}}$ Primärenergie $Q_{{P}}$."),'),
 ('         r"The gas boiler, $154\,\mathrm{kWh/(m^2\,a)}$, misses it by more than a factor of two.",\n         r"Der Gaskessel verfehlt sie mit $154\,\mathrm{kWh/(m^2\,a)}$ um mehr als das Doppelte."),',
  '         rf"The gas boiler, ${Q_PRIM_GAS}\,\mathrm{{kWh/(m^2\,a)}}$, misses it by more than a factor of two.",\n         rf"Der Gaskessel verfehlt sie mit ${Q_PRIM_GAS}\,\mathrm{{kWh/(m^2\,a)}}$ um mehr als das Doppelte."),'),
]
for old, new in pairs:
    print(s.count(old))
    s = s.replace(old, new)
path.write_text(s, encoding="utf-8", newline="\n")
