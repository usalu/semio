import importlib

R = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🔁️recap.🐍️")
for k in ("A_WINDOW", "G_VALUE", "F_S", "I_S", "Q_SOLAR", "Q_INTERN", "Q_L", "Q_G", "ETA_H", "Q_USABLE", "Q_L_JAN", "Q_G_JAN", "ETA_JAN", "Q_USABLE_JAN", "ETA_C"):
    print(k, getattr(R, k, None))
print([n for n in dir(R) if n.startswith("Q_") or n.startswith("ETA")])
for k in ("Q_G_JUL", "Q_L_JUL"):
    print(k, getattr(R, k))
print("qh", R.Q_L - R.ETA_H * R.Q_G, R.Q_L_JAN - R.ETA_JAN * R.Q_G_JAN, "qc", R.Q_G_JUL - R.ETA_C * R.Q_L_JUL)
