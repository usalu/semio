#!/usr/bin/env python3
"""Language-agnostic EN 1990 combination oracle (kN-scale ActionSet)."""

def combination_6_10a(g_k, q_k, leading, gamma_g=1.35, gamma_q=1.5, psi0=None):
    s = gamma_g * g_k
    for i, (cat, q) in enumerate(q_k):
        p0 = psi0(cat) if psi0 else 0.7
        s += (gamma_q if i == leading else gamma_q * p0) * q
    return s

def combination_6_10b(g_k, q_k, leading, xi=0.85, gamma_g=1.35, gamma_q=1.5, psi0=None):
    s = xi * gamma_g * g_k
    for i, (cat, q) in enumerate(q_k):
        p0 = psi0(cat) if psi0 else 0.7
        s += (gamma_q if i == leading else gamma_q * p0) * q
    return s

def combination_6_10(g_k, q_k, leading, **kw):
    return max(combination_6_10a(g_k, q_k, leading, **kw), combination_6_10b(g_k, q_k, leading, **kw))

def psi0_de(cat):
    return {"office": 0.7, "wind": 0.6, "other": 0.8, "snow": 0.5, "snow_high": 0.7}.get(cat, 0.7)

def psi0_en(cat):
    return {"office": 0.7, "wind": 0.6, "other": 0.7, "snow": 0.5}.get(cat, 0.7)

def k_fi(cc):
    return {1: 0.9, 3: 1.1}.get(cc, 1.0)

def beta_target(rc, t_years):
    fifty = t_years >= 25
    return {(1, True): 3.3, (1, False): 4.2, (3, True): 4.3, (3, False): 5.2}.get((rc, fifty), 3.8 if fifty else 4.7)

if __name__ == "__main__":
    q = [("office", 50.0), ("wind", 30.0)]
    assert abs(combination_6_10a(100, q, 0, psi0=psi0_de) - 237.0) < 1e-9
    assert abs(combination_6_10b(100, q, 0, psi0=psi0_de) - 216.75) < 1e-9
    assert abs(combination_6_10(100, q, 0, psi0=psi0_de) - 237.0) < 1e-9
    assert abs(k_fi(3) - 1.1) < 1e-9
    assert abs(beta_target(3, 50) - 4.3) < 1e-9
    print("oracle self-check ok")
