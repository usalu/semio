"""electric-kick-scooter: front and plan views at 1:200, 1:100, 1:50 (1:500 stays unavailable)."""
from lib import *

AID = "electric-kick-scooter"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)


WF, WR = 0.43, -0.43
TW = 0.0225
TT, TR = "tyres", "rubber"

# ---------------------------------------------------------------- 1:50
front = [
    S("tyre-r", "wheel-r", "body", 3.9, rect(-TW, 0, 2 * TW, 0.216, 0.02), 0.1, region=TT, material=TR),
    S("fender-r", "deck", "body", 3.5, rect(-0.03, 0.128, 0.06, 0.106, 0.012), 0.55, region="fenders", material="plastic"),
    S("hub-r", "wheel-r", "detail", 4.1, rect(TW, 0.078, 0.02, 0.06, 0.004), 0.45, region="brake-disc", material="steel-disc"),
    S("deck", "deck", "body", 3, rect(-0.085, 0.095, 0.17, 0.05, 0.008), 0.3, region="deck-battery", material="anodised"),
    S("grip-tape", "deck", "body", 3.2, rect(-0.07, 0.135, 0.14, 0.023, 0.004), 0.12, region="deck-grip", material="griptape"),
    S("tyre-f", "wheel-f", "body", 4, rect(-TW, 0, 2 * TW, 0.216, 0.02), 0.1, region=TT, material=TR),
    S("hub-f", "wheel-f", "detail", 4.8, rect(TW, 0.073, 0.022, 0.07, 0.006), 0.45, region="hub-motor", material="motor-alloy"),
    S("fork-blade", "fork", "body", 5, poly([(0, 0.305), (0, 0.272), (0.0445, 0.272), (0.0445, 0.108), (0.062, 0.108), (0.062, 0.305)]) | {"followTip": True}, 0.4, region="stem-bars", material="anodised"),
    S("head-block", "fork", "body", 3.6, rect(-0.035, 0.25, 0.07, 0.08, 0.006), 0.3, region="stem-bars", material="anodised"),
    S("stem", "stem", "body", 3.5, rect(-0.019, 0.33, 0.038, 0.762, 0.01, True), 0.4, region="stem-bars", material="anodised"),
    S("bar", "bars", "body", 3.6, rect(-0.19, 1.078, 0.38, 0.028, 0.012), 0.4, region="stem-bars", material="anodised"),
    S("grip-l", "bars", "body", 3.7, rect(0.12, 1.062, 0.095, 0.056, 0.026), 0.1, region="stem-bars", material="rubber"),
    S("grip-r", "bars", "body", 3.7, rect(-0.215, 1.062, 0.095, 0.056, 0.026), 0.1, region="stem-bars", material="rubber"),
    S("brake-lever", "bars", "detail", 3.8, rect(0.168, 1.04, 0.02, 0.05, 0.006), 0.2, region="stem-bars", material="anodised"),
]
plan = [
    S("tyre-r", "wheel-r", "body", 4, rect(-0.538, -TW, 0.216, 2 * TW, 0.012), 0.1, region=TT, material=TR),
    S("disc-r", "wheel-r", "detail", 4.1, rect(-0.46, TW, 0.06, 0.006), 0.45, region="brake-disc", material="steel-disc"),
    S("deck", "deck", "body", 3, rect(-0.41, -0.085, 0.79, 0.17, 0.02), 0.3, region="deck-battery", material="anodised"),
    S("grip-tape", "deck", "body", 3.2, rect(-0.36, -0.07, 0.66, 0.14, 0.012), 0.12, region="deck-grip", material="griptape"),
    S("fender-r", "deck", "body", 4.5, rect(-0.554, -0.03, 0.246, 0.06, 0.02), 0.55, region="fenders", material="plastic"),
    S("tyre-f", "wheel-f", "body", 4, rect(0.322, -TW, 0.216, 2 * TW, 0.012), 0.1, region=TT, material=TR),
    S("hub-f", "wheel-f", "detail", 4.8, rect(0.4, TW, 0.07, 0.022, 0.006), 0.45, region="hub-motor", material="motor-alloy"),
    S("fork-blade", "fork", "body", 5, cap(0.4, 0.0535, 0.43, 0.0535, 0.0085, follow=True), 0.4, region="stem-bars", material="anodised"),
    S("fork-crown", "fork", "body", 3.7, cap(0.4, 0, 0.4, 0.0535, 0.015), 0.4, region="stem-bars", material="anodised"),
    S("head-block", "fork", "body", 3.6, rect(0.365, -0.035, 0.07, 0.07, 0.008), 0.3, region="stem-bars", material="anodised"),
    S("stem", "stem", "body", 3.5, cap(0.4, 0, 0.289, 0, 0.019, 0.015, True), 0.4, region="stem-bars", material="anodised"),
    S("bar", "bars", "body", 3.6, cap(0.289, -0.187, 0.289, 0.187, 0.014), 0.4, region="stem-bars", material="anodised"),
    S("grip-l", "bars", "body", 3.7, cap(0.289, 0.12, 0.289, 0.187, 0.028), 0.1, region="stem-bars", material="rubber"),
    S("grip-r", "bars", "body", 3.7, cap(0.289, -0.187, 0.289, -0.12, 0.028), 0.1, region="stem-bars", material="rubber"),
    S("brake-lever", "bars", "detail", 3.8, cap(0.3, 0.17, 0.39, 0.17, 0.008), 0.2, region="stem-bars", material="anodised"),
]
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: tyre, deck, hub cap, one-sided fork, stem and handlebar; top: both tyres, deck with grip tape, fender, fork, stem and handlebar with grips.",
                   "Vorn: Reifen, Trittbrett, Nabenkappe, einseitige Gabel, Vorbau und Lenker; Oben: beide Reifen, Trittbrett mit Griptape, Schutzblech, Gabel, Vorbau und Lenker mit Griffen."),
    "overlays": ("Front and top shapes carry the regions of the side view. The stem and fork blade are bound with followTip (front rects, top capsule) and the handlebar to the added bone bars at the stem tip, so folding and braking move them correctly.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht. Vorbau und Gabelholm sind mit followTip gebunden (Rechtecke vorn, Kapsel oben) und der Lenker an den ergänzten Knochen bars an der Vorbauspitze, so folgen sie dem Klappen und Bremsen."),
})

# ---------------------------------------------------------------- 1:100
front = [
    S("tyre-r", "wheel-r", "body", 3.9, rect(-TW, 0, 2 * TW, 0.216, 0.02), 0.1, region=TT, material=TR),
    S("fender-r", "deck", "body", 3.5, rect(-0.03, 0.128, 0.06, 0.092, 0.012), 0.55, region="fenders", material="plastic"),
    S("deck", "deck", "body", 3, rect(-0.085, 0.095, 0.17, 0.05, 0.008), 0.3, regions=["deck-battery", "deck-grip", "bms-controller"], material="anodised"),
    S("tyre-f", "wheel-f", "body", 4, rect(-TW, 0, 2 * TW, 0.216, 0.02), 0.1, regions=[TT, "hub-motor"], material=TR),
    S("fork-blade", "fork", "body", 5, poly([(0, 0.305), (0, 0.272), (0.0445, 0.272), (0.0445, 0.108), (0.062, 0.108), (0.062, 0.305)]) | {"followTip": True}, 0.35, region="stem-bars", material="anodised"),
    S("stem", "stem", "body", 3.5, rect(-0.02, 0.33, 0.04, 0.762, 0.01, True), 0.35, region="stem-bars", material="anodised"),
    S("bar", "bars", "body", 3.6, rect(-0.19, 1.078, 0.38, 0.028, 0.012), 0.35, region="stem-bars", material="anodised"),
    S("grip-l", "bars", "body", 3.7, rect(0.12, 1.062, 0.095, 0.056, 0.026), 0.1, region="stem-bars", material="rubber"),
    S("grip-r", "bars", "body", 3.7, rect(-0.215, 1.062, 0.095, 0.056, 0.026), 0.1, region="stem-bars", material="rubber"),
]
plan = [
    S("tyre-r", "wheel-r", "body", 4, rect(-0.538, -TW, 0.216, 2 * TW, 0.012), 0.1, regions=[TT, "brake-disc"], material=TR),
    S("deck", "deck", "body", 3, rect(-0.41, -0.085, 0.79, 0.17, 0.02), 0.3, regions=["deck-battery", "deck-grip", "bms-controller"], material="anodised"),
    S("fender-r", "deck", "body", 4.5, rect(-0.54, -0.03, 0.232, 0.06, 0.02), 0.55, region="fenders", material="plastic"),
    S("tyre-f", "wheel-f", "body", 4, rect(0.322, -TW, 0.216, 2 * TW, 0.012), 0.1, regions=[TT, "hub-motor"], material=TR),
    S("fork-blade", "fork", "body", 5, cap(0.4, 0.0535, 0.43, 0.0535, 0.009, follow=True), 0.35, region="stem-bars", material="anodised"),
    S("fork-crown", "fork", "body", 3.7, rect(0.365, -0.035, 0.07, 0.07, 0.008), 0.35, region="stem-bars", material="anodised"),
    S("stem", "stem", "body", 3.5, cap(0.4, 0, 0.289, 0, 0.019, 0.015, True), 0.35, region="stem-bars", material="anodised"),
    S("bar", "bars", "body", 3.6, cap(0.289, -0.187, 0.289, 0.187, 0.014), 0.35, region="stem-bars", material="anodised"),
    S("grip-l", "bars", "body", 3.7, cap(0.289, 0.12, 0.289, 0.187, 0.028), 0.1, region="stem-bars", material="rubber"),
    S("grip-r", "bars", "body", 3.7, cap(0.289, -0.187, 0.289, -0.12, 0.028), 0.1, region="stem-bars", material="rubber"),
]
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top view drawn as articulated masses: tyres, deck, fender, fork blade, stem, bar and grips.",
                   "Vorder- und Draufsicht als gegliederte Massen gezeichnet: Reifen, Trittbrett, Schutzblech, Gabelholm, Vorbau, Lenkerstange und Griffe."),
    "merged": ("Front and top: deck, grip tape and controller are one shape; hub motor and brake disc sit inside their tyre shapes.",
               "Vorn und oben: Trittbrett, Griptape und Steuerung sind eine Form; Nabenmotor und Bremsscheibe liegen in ihren Reifenformen."),
})

# ---------------------------------------------------------------- 1:200
front = [
    S("wheel-r", "wheel-r", "body", 3.9, rect(-TW, 0, 2 * TW, 0.216, 0.02), 0.1, regions=[TT, "brake-disc"], material=TR),
    S("deck", "deck", "body", 3, rect(-0.085, 0.095, 0.17, 0.05), 0.3, regions=["deck-battery", "deck-grip", "bms-controller", "fenders"], material="anodised"),
    S("wheel-f", "wheel-f", "body", 4, rect(-TW, 0, 2 * TW, 0.216, 0.02), 0.1, regions=[TT, "hub-motor"], material=TR),
    S("fork-blade", "fork", "body", 5, rect(0.0, 0.108, 0.06, 0.197, None, True), 0.3, region="stem-bars", material="anodised"),
    S("fork-stem", "stem", "body", 3.5, rect(-0.025, 0.3, 0.05, 0.792, None, True), 0.3, region="stem-bars", material="anodised"),
    S("bars", "bars", "body", 3.7, rect(-0.215, 1.063, 0.43, 0.058, 0.02), 0.2, region="stem-bars", material="rubber"),
]
plan = [
    S("wheel-r", "wheel-r", "body", 4, rect(-0.538, -TW, 0.216, 2 * TW, 0.01), 0.1, regions=[TT, "brake-disc"], material=TR),
    S("deck", "deck", "body", 3, rect(-0.41, -0.085, 0.79, 0.17, 0.02), 0.3, regions=["deck-battery", "deck-grip", "bms-controller", "fenders"], material="anodised"),
    S("wheel-f", "wheel-f", "body", 4, rect(0.322, -TW, 0.216, 2 * TW, 0.01), 0.1, regions=[TT, "hub-motor"], material=TR),
    S("fork-blade", "fork", "body", 5, cap(0.4, 0.04, 0.43, 0.04, 0.012, follow=True), 0.3, region="stem-bars", material="anodised"),
    S("fork-stem", "stem", "body", 3.5, cap(0.4, 0, 0.289, 0, 0.022, follow=True), 0.3, region="stem-bars", material="anodised"),
    S("bars", "bars", "body", 3.7, cap(0.289, -0.187, 0.289, 0.187, 0.028), 0.2, region="stem-bars", material="rubber"),
]
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top view show only the main masses: two tyre bodies, deck, one-sided fork blade, stem and a single handlebar bar.",
                   "Vorder- und Draufsicht zeigen nur die Hauptmassen: zwei Reifenkörper, Trittbrett, einseitigen Gabelholm, Vorbau und eine einzige Lenkerstange."),
    "merged": ("Front and top: grips and bar are one shape; fender, grip tape and controller are part of the deck shape.",
               "Vorn und oben: Griffe und Stange sind eine Form; Schutzblech, Griptape und Steuerung gehören zur Trittbrettform."),
})

STEM = cap(0, 0, 0.8, 0, 0.019, 0.015)
BLADE = cap(0, 0, 0.03, -0.192, 0.0085)


def kick(grip_r, lever=True):
    f = {"stem": ("stem", ext(STEM, -0.019, 0.019)), "fork-blade": ("fork", ext(BLADE, 0.045, 0.062)),
         "bar": ("stem", ext(rect(0.785, -0.014, 0.03, 0.028, 0.012), -0.19, 0.19)),
         "grip-l": ("stem", ext(ell(0.8, 0, grip_r), 0.12, 0.215)), "grip-r": ("stem", ext(ell(0.8, 0, grip_r), -0.215, -0.12))}
    if lever:
        f["brake-lever"] = ("stem", ext(cap(0.8036, -0.0106, 0.7762, -0.0976, 0.008), 0.16, 0.18))
    return f


apply_rules(a, {
    "50": {"front": kick(0.028), "plan": kick(0.028)},
    "100": {"front": kick(0.028, False), "plan": kick(0.028, False)},
    "200": {
        "front": {"fork-stem": ("stem", ext(cap(0, 0, 0.8, 0, 0.025), -0.025, 0.025)), "fork-blade": ("fork", ext(BLADE, 0.0, 0.06)), "bars": ("stem", ext(rect(0.77, -0.029, 0.06, 0.058, 0.02), -0.215, 0.215))},
        "plan": {"fork-stem": ("stem", ext(cap(0, 0, 0.8, 0, 0.022), -0.022, 0.022)), "fork-blade": ("fork", ext(cap(0, 0, 0.03, -0.192, 0.012), 0.028, 0.052)), "bars": ("stem", ext(rect(0.77, -0.029, 0.06, 0.058, 0.02), -0.215, 0.215))},
    },
})

save(AID, doc)
for s in ("50", "100", "200"):
    print(report(a, s))
