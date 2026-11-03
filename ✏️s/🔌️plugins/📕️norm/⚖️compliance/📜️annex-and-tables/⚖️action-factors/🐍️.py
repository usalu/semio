#!/usr/bin/env python3
"""⚖️ Partial and combination factors on actions for the Python oracles: the same registry rows `ndp.en1990.{gG_sup,gG_inf,gQ,gP_*,xi,psi,kfi}.*` the Rust `ActionFactors` reads (`🦀️.rs` next to this file), so an oracle never keeps a local copy of γ_G, γ_Q, ξ, ψ or K_FI.

Load it from an oracle with `importlib.util.spec_from_file_location("action_factors", path)` where `path` is the first parent's `⚖️compliance/📜️annex-and-tables/⚖️action-factors/🐍️.py`; it uses no dataclass so it needs no `sys.modules` entry.

Links: https://eurocodes.jrc.ec.europa.eu/EN-Eurocodes/eurocode-0-basis-structural-design
"""
from __future__ import annotations

import json
from pathlib import Path

REGISTRY = Path(__file__).resolve().parents[1] / "🔖️ndp" / "🔣️.json"
_NDP: dict = {}


def annex_of(annex: str) -> str:
    """🌍️ `de` or `en` from any annex spelling (`De`, `DE-NA`, `din`, `En`)."""
    return "de" if str(annex or "").strip().lower() in ("de", "de-na", "din") else "en"


def ndp(row_id: str, annex: str) -> float:
    """📖 Value of the full registry row id (`ndp.en1990.gG_sup.str.A1`) for the annex; raises for an unknown or non-numeric row."""
    if not _NDP:
        _NDP.update({e["id"]: e for e in json.loads(REGISTRY.read_text(encoding="utf-8"))["entries"]})
    value = _NDP[row_id]["naDe" if annex_of(annex) == "de" else "recommended"]
    if value is None:
        raise KeyError(f"{row_id} has no number for {annex}")
    return float(value)


def canonical_category(key: str) -> str:
    """🔤️ The registry key of an imposed-load category such as `B` or `B2`; other keys stay as they are."""
    if len(key) in (1, 2) and (len(key) == 1 or key[1].isdigit()):
        return {"A": "residential", "T": "residential", "Z": "residential", "B": "office", "C": "congregation", "D": "retail", "E": "storage", "F": "traffic_light", "G": "traffic_heavy", "H": "roof"}.get(key[0], key)
    return key


class ActionFactors:
    """⚖️ γ_G,sup, γ_G,inf, γ_Q, γ_P, ξ, ψ and K_FI of one annex, reliability class, design situation and limit state (Annex A1 buildings, Table A1.2(B) unless `domain` and `factor_set` say otherwise)."""

    def __init__(self, annex: str, reliability_class: int = 2, situation: str = "persistent", serviceability: bool = False, domain: str = "A1", factor_set: str = "str", k_fi_value: float | None = None):
        self.annex, self.reliability_class, self.situation, self.serviceability = annex, reliability_class, situation, serviceability
        self.domain, self.factor_set, self.k_fi_value = domain, factor_set, k_fi_value

    @property
    def k_fi(self) -> float:
        return self.k_fi_value if self.k_fi_value is not None else ndp(f"ndp.en1990.kfi.{self.reliability_class}", self.annex)

    @property
    def exceptional(self) -> bool:
        return self.situation in ("accidental", "seismic")

    @property
    def fundamental(self) -> bool:
        return not self.serviceability and not self.exceptional

    def gamma_g_sup(self) -> float:
        return self.k_fi * ndp(f"ndp.en1990.gG_sup.{self.factor_set}.{self.domain}", self.annex) if self.fundamental else 1.0

    def gamma_g_inf(self) -> float:
        return ndp(f"ndp.en1990.gG_inf.{self.factor_set}.{self.domain}", self.annex) if self.fundamental else 1.0

    def xi(self) -> float:
        return ndp(f"ndp.en1990.xi.str.{self.domain}", self.annex) if self.fundamental else 1.0

    def gamma_q(self, category: str = "") -> float:
        """🔢️ γ_Q times K_FI; bridges (domain `A2`) split road and footbridge, rail and other variable actions by `category`."""
        if not self.fundamental:
            return 1.0
        if self.domain == "A2":
            family = "road" if category.startswith(("road_", "footbridge_")) else "rail" if category.startswith("rail_") else "other"
            return self.k_fi * ndp(f"ndp.en1990.gQ.{family}.{self.factor_set}.A2", self.annex)
        return self.k_fi * ndp(f"ndp.en1990.gQ.{self.factor_set}.A1", self.annex)

    def gamma_p(self, relieving: bool = False) -> float:
        return ndp("ndp.en1990.gP_inf" if relieving else "ndp.en1990.gP_sup", self.annex) if self.fundamental else 1.0

    def psi_known(self, category: str) -> tuple[float, float, float] | None:
        """🔎️ (ψ0, ψ1, ψ2) of the category, `None` when the registry has no row for it."""
        key = canonical_category(category)
        try:
            return tuple(ndp(f"ndp.en1990.psi.{key}.{i}", self.annex) for i in range(3))
        except KeyError:
            return None

    def psi(self, category: str) -> tuple[float, float, float]:
        """🔎️ (ψ0, ψ1, ψ2) of the category; an empty or unknown category is (1, 1, 1)."""
        return self.psi_known(category) or (1.0, 1.0, 1.0)

    def in_situation(self, situation: str) -> "ActionFactors":
        return ActionFactors(self.annex, self.reliability_class, situation, self.serviceability, self.domain, self.factor_set, self.k_fi_value)
