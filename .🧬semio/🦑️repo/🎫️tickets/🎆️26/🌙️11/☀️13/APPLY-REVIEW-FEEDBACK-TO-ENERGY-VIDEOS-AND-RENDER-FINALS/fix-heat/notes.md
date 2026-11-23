# fix-heat working notes (sheets: 3x3, 4 s per frame; sheet n starts at n*36 s)

Sheets 24..51 (14:24 to 31:08) reviewed:
- 15:56-16:00 (s_26): formula box "psi_i - psi_e = U.2d = ..." drawn over the psi_e/psi_i rows (rows still visible) -> check 1 fps
- 16:32-16:52 (s_27/28): H_T formula garbled "HH_T=SUMUUAA+DUWB..." and "[H]=WW KK" doubled, held 20 s -> REAL leftover after morph
- 18:56-19:04 (s_31): formula box [Qi,p [Qi]=W] hidden under 2-line caption -> REAL
- 20:12-20:20 (s_33): box q_i = Qi/A_N [q_i]=W/m2 under 2-line caption -> REAL
- 24:44-25:12 (s_41/42): leftover symbol row I_s A g F.. behind Q_Speicher formula -> REAL
- 23:56/24:00: "20 m2 * 0,133 kWh = 2,7 kWh" missing /m2 on 0,133 kWh
- 26:52-27:08 (s_44/45): Q_l = (H_T+H_V) G... overlapped with old formula -> REAL leftover
- 27:12-27:52 (s_45/46): gray subtitle "e Waermegewinne (DIN V 4108-6:2003-06)" clipped at left frame edge -> REAL
- 25:24-26:00 uses rho c_p while 17:xx uses c_Luft (minor)
- 29:44-31:00: house touches left frame edge (5px at 640) minor
Numbers recomputed OK: 2311 W table, H_T 115.6, 41/20.5 W/K, psi values, 768 W -> 232 W, 16000/4500/11500, Q_E 14100.
