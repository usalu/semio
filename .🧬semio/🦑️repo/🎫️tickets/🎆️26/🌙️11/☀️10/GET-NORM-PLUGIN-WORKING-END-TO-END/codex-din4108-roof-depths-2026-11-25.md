# DIN 4108 roof temperature coordinates

The default roof includes a 0.20 mm PE vapour-control membrane. The depth calculation for the temperature diagram reused the wall-section rendering helper, which gives each visible layer at least 1.00 mm. This increased the membrane's diagram depth and every later interface by 0.80 mm. The thermal resistance calculation and independent NumPy oracle already used the thickness stored in the document.

The temperature series, table and interface markers now share cumulative depths from the stored layer thicknesses. The roof coordinates from the interior surface are 0, 0.0125, 0.0127, 0.2127 and 0.3727 m. The drawing floor for the 3D wall assembly and the numeric oracle are unchanged.

A focused Python comparison read the four layers from the actual default roof, calculated exact decimal thickness sums, and computed temperatures from the series-resistance ratios separately from the NumPy nodal solve. The existing oracle rejected the former coordinates at the three same interfaces reported by the native suite. It accepted the physical coordinates in all ten roof temperature and depth checks. The parent owns the final native suite rerun.
