"""🏷️ EN 1993-1-1 oracle for Table 5.2 classification (V-028, V-030..V-033), shear resistance 6.2.6 (V-063) and the Annex A interaction (V-120): closed forms written from the standard, section area from the third-party `sectionproperties` package."""

import json
import math

E = 210000.0


def eps(fy):
    return math.sqrt(235.0 / fy)


def internal(ct, e, compression):
    limits = (33 * e, 38 * e, 42 * e) if compression else (72 * e, 83 * e, 124 * e)
    return 1 + sum(ct > limit for limit in limits)


def outstand(ct, e):
    return 1 + sum(ct > limit for limit in (9 * e, 10 * e, 14 * e))


def chs(dt, e):
    return 1 + sum(dt > limit for limit in (50 * e * e, 70 * e * e, 90 * e * e))


def angle(h, b, t, e):
    return 3 if h / t <= 15 * e and (b + h) / (2 * t) <= 11.5 * e else 4


def ipe300_area():
    from sectionproperties.pre.library import i_section
    from sectionproperties.analysis import Section

    geometry = i_section(d=300.0, b=150.0, t_f=10.7, t_w=7.1, r=15.0, n_r=32)
    geometry.create_mesh(mesh_sizes=[20.0])
    section = Section(geometry)
    section.calculate_geometric_properties()
    return section.get_area()


def shear(area):
    b, tf, tw, r, h, fy = 150.0, 10.7, 7.1, 15.0, 300.0, 355.0
    hw = h - 2 * tf
    av = max(area - 2 * b * tf + (tw + 2 * r) * tf, 1.2 * hw * tw)
    return av, av * fy / math.sqrt(3.0)


def chi_curve(lam, alpha):
    phi = 0.5 * (1 + alpha * (lam - 0.2) + lam * lam)
    return min(1.0, 1.0 / (phi + math.sqrt(phi * phi - lam * lam)))


def annex_a():
    fy, a, iy, iz = 355.0, 5381.0, 8356e4, 603.8e4
    wel_y, wpl_y, wel_z, wpl_z = 557e3, 628.4e3, 80.5e3, 125.2e3
    length, n, my, mz = 4000.0, 400e3, 100e6, 0.0
    ncr_y, ncr_z = math.pi ** 2 * E * iy / length ** 2, math.pi ** 2 * E * iz / length ** 2
    n_rk, my_rk, mz_rk = a * fy, wpl_y * fy, wpl_z * fy
    lam_y, lam_z = math.sqrt(n_rk / ncr_y), math.sqrt(n_rk / ncr_z)
    chi_y, chi_z = chi_curve(lam_y, 0.21), chi_curve(lam_z, 0.34)
    gm1, cmy0, cmz0 = 1.0, 0.9, 0.9
    wy, wz = min(wpl_y / wel_y, 1.5), min(wpl_z / wel_z, 1.5)
    npl, lmax = n / (n_rk / gm1), max(lam_y, lam_z)
    mu_y = (1 - n / ncr_y) / (1 - chi_y * n / ncr_y)
    mu_z = (1 - n / ncr_z) / (1 - chi_z * n / ncr_z)
    cyy = max(1 + (wy - 1) * ((2 - 1.6 / wy * cmy0 ** 2 * lmax - 1.6 / wy * cmy0 ** 2 * lmax ** 2) * npl), wel_y / wpl_y)
    cyz = max(1 + (wz - 1) * ((2 - 14 * cmz0 ** 2 * lmax ** 2 / wz ** 5) * npl), 0.6 * math.sqrt(wz / wy) * wel_z / wpl_z)
    czy = max(1 + (wy - 1) * ((2 - 14 * cmy0 ** 2 * lmax ** 2 / wy ** 5) * npl), 0.6 * math.sqrt(wy / wz) * wel_y / wpl_y)
    czz = max(1 + (wz - 1) * ((2 - 1.6 / wz * cmz0 ** 2 * lmax - 1.6 / wz * cmz0 ** 2 * lmax ** 2) * npl), wel_z / wpl_z)
    kyy = cmy0 * mu_y / (1 - n / ncr_y) / cyy
    kyz = cmz0 * mu_y / (1 - n / ncr_z) / cyz * 0.6 * math.sqrt(wz / wy)
    kzy = cmy0 * mu_z / (1 - n / ncr_y) / czy * 0.6 * math.sqrt(wy / wz)
    kzz = cmz0 * mu_z / (1 - n / ncr_z) / czz
    eta61 = n / (chi_y * n_rk / gm1) + kyy * my / (my_rk / gm1) + kyz * mz / (mz_rk / gm1)
    eta62 = n / (chi_z * n_rk / gm1) + kzy * my / (my_rk / gm1) + kzz * mz / (mz_rk / gm1)
    return dict(ncr_y=ncr_y, ncr_z=ncr_z, chi_y=chi_y, chi_z=chi_z, lambda_y=lam_y, lambda_z=lam_z, kyy=kyy, kyz=kyz, kzy=kzy, kzz=kzz, eta_61=eta61, eta_62=eta62)


def main():
    e = eps(355.0)
    area = ipe300_area()
    av, vpl = shear(area)
    web = 300.0 - 2 * 10.7 - 2 * 15.0
    flange = (150.0 - 7.1 - 2 * 15.0) / 2
    out = dict(
        epsilon_355=e,
        area=area,
        web_ct=web / 7.1,
        flange_ct=flange / 10.7,
        class_web_compression=internal(web / 7.1, e, True),
        class_web_bending=internal(web / 7.1, e, False),
        class_flange=outstand(flange / 10.7, e),
        class_plate_outstand_10=outstand(10.0, e),
        class_plate_outstand_12=outstand(12.0, e),
        class_chs_139_5=chs(139.7 / 5.0, e),
        class_chs_273_63=chs(273.0 / 6.3, e),
        class_chs_508_5=chs(508.0 / 5.0, e),
        class_angle_s235=angle(100.0, 100.0, 10.0, 1.0),
        class_angle_s355=angle(100.0, 100.0, 10.0, e),
        class_section_compression=max(internal(web / 7.1, e, True), outstand(flange / 10.7, e)),
        a_v=av,
        v_pl_rd=vpl,
        annex_a=annex_a(),
    )
    out.update(out["annex_a"])
    print(json.dumps(out))


if __name__ == "__main__":
    main()
