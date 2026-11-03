//! 👁️ Colour metrics: WCAG contrast, CIE L*a*b* distance and colour-vision-deficiency simulation (Machado et al. 2009, severity 1.0).

pub type Srgb = [f64; 3];

/// 🔍️ Colour-vision deficiency a palette must stay distinguishable under.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cvd {
    Protanopia,
    Deuteranopia,
    Tritanopia,
}

pub const ALL_CVD: [Cvd; 3] = [Cvd::Protanopia, Cvd::Deuteranopia, Cvd::Tritanopia];

impl Cvd {
    pub fn id(self) -> &'static str {
        match self {
            Self::Protanopia => "prot",
            Self::Deuteranopia => "deut",
            Self::Tritanopia => "trit",
        }
    }

    fn matrix(self) -> [[f64; 3]; 3] {
        match self {
            Self::Protanopia => [[0.152286, 1.052583, -0.204868], [0.114503, 0.786281, 0.099216], [-0.003882, -0.048116, 1.051998]],
            Self::Deuteranopia => [[0.367322, 0.860646, -0.227968], [0.280085, 0.672501, 0.047413], [-0.011820, 0.042940, 0.968881]],
            Self::Tritanopia => [[1.255528, -0.076749, -0.178779], [-0.078411, 0.930809, 0.147602], [0.004733, 0.691367, 0.303900]],
        }
    }
}

pub fn to_srgb(rgb: [u8; 3]) -> Srgb {
    rgb.map(f64::from)
}

fn linear(c: f64) -> f64 {
    let c = c / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

fn encode(v: f64) -> f64 {
    255.0 * if v <= 0.0031308 { 12.92 * v } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 }
}

/// 💡️ WCAG relative luminance.
pub fn luminance(c: Srgb) -> f64 {
    0.2126 * linear(c[0]) + 0.7152 * linear(c[1]) + 0.0722 * linear(c[2])
}

/// 🌗️ WCAG 2 contrast ratio, 1..21.
pub fn contrast_ratio(a: Srgb, b: Srgb) -> f64 {
    let (la, lb) = (luminance(a) + 0.05, luminance(b) + 0.05);
    la.max(lb) / la.min(lb)
}

/// 🧪️ Colour as seen under a deficiency.
pub fn simulate(cvd: Cvd, c: Srgb) -> Srgb {
    let l = c.map(linear);
    let m = cvd.matrix();
    std::array::from_fn(|i| encode((m[i][0] * l[0] + m[i][1] * l[1] + m[i][2] * l[2]).clamp(0.0, 1.0)))
}

/// 🧮️ CIE L*a*b* (D65) of an sRGB colour.
pub fn lab(c: Srgb) -> [f64; 3] {
    let [r, g, b] = c.map(linear);
    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;
    let f = |t: f64| if t > 216.0 / 24389.0 { t.cbrt() } else { (24389.0 / 27.0 * t + 16.0) / 116.0 };
    let (fx, fy, fz) = (f(x / 0.95047), f(y), f(z / 1.08883));
    [116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz)]
}

/// 📏️ CIE76 colour difference.
pub fn delta_e(a: Srgb, b: Srgb) -> f64 {
    let (la, lb) = (lab(a), lab(b));
    ((la[0] - lb[0]).powi(2) + (la[1] - lb[1]).powi(2) + (la[2] - lb[2]).powi(2)).sqrt()
}

/// 👓️ Smallest colour difference between neighbouring samples under a deficiency.
pub fn min_adjacent_delta_e(cvd: Cvd, samples: &[[u8; 3]]) -> f64 {
    let seen: Vec<Srgb> = samples.iter().map(|s| simulate(cvd, to_srgb(*s))).collect();
    seen.windows(2).map(|w| delta_e(w[0], w[1])).fold(f64::INFINITY, f64::min)
}

/// 📏️ CIEDE2000 colour difference of two L*a*b* colours (Sharma, Wu and Dalal 2005); the perceptual measure used to judge palettes under colour-vision deficiency.
pub fn delta_e2000_lab(first: [f64; 3], second: [f64; 3]) -> f64 {
    let [l1, a1, b1] = first;
    let [l2, a2, b2] = second;
    let chroma_mean = (a1.hypot(b1) + a2.hypot(b2)) / 2.0;
    let g = 0.5 * (1.0 - (chroma_mean.powi(7) / (chroma_mean.powi(7) + 25f64.powi(7))).sqrt());
    let (ap1, ap2) = ((1.0 + g) * a1, (1.0 + g) * a2);
    let (cp1, cp2) = (ap1.hypot(b1), ap2.hypot(b2));
    let hue = |b: f64, a: f64| if a == 0.0 && b == 0.0 { 0.0 } else { b.atan2(a).to_degrees().rem_euclid(360.0) };
    let (hp1, hp2) = (hue(b1, ap1), hue(b2, ap2));
    let (dl, dc) = (l2 - l1, cp2 - cp1);
    let raw = hp2 - hp1;
    let dh = if cp1 * cp2 == 0.0 { 0.0 } else if raw > 180.0 { raw - 360.0 } else if raw < -180.0 { raw + 360.0 } else { raw };
    let dhh = 2.0 * (cp1 * cp2).sqrt() * (dh.to_radians() / 2.0).sin();
    let (lb, cpb) = ((l1 + l2) / 2.0, (cp1 + cp2) / 2.0);
    let hb = if cp1 * cp2 == 0.0 { hp1 + hp2 } else if (hp1 - hp2).abs() <= 180.0 { (hp1 + hp2) / 2.0 } else if hp1 + hp2 < 360.0 { (hp1 + hp2 + 360.0) / 2.0 } else { (hp1 + hp2 - 360.0) / 2.0 };
    let cos = |degrees: f64| degrees.to_radians().cos();
    let t = 1.0 - 0.17 * cos(hb - 30.0) + 0.24 * cos(2.0 * hb) + 0.32 * cos(3.0 * hb + 6.0) - 0.20 * cos(4.0 * hb - 63.0);
    let rc = 2.0 * (cpb.powi(7) / (cpb.powi(7) + 25f64.powi(7))).sqrt();
    let rt = -(60.0 * (-((hb - 275.0) / 25.0).powi(2)).exp()).to_radians().sin() * rc;
    let sl = 1.0 + 0.015 * (lb - 50.0).powi(2) / (20.0 + (lb - 50.0).powi(2)).sqrt();
    let (sc, sh) = (1.0 + 0.045 * cpb, 1.0 + 0.015 * cpb * t);
    ((dl / sl).powi(2) + (dc / sc).powi(2) + (dhh / sh).powi(2) + rt * (dc / sc) * (dhh / sh)).sqrt()
}

/// 📏️ CIEDE2000 difference of two sRGB colours.
pub fn delta_e2000(a: Srgb, b: Srgb) -> f64 {
    delta_e2000_lab(lab(a), lab(b))
}

/// 👓️ Smallest CIEDE2000 difference over all pairs of colours as seen under a deficiency.
pub fn min_pairwise_delta_e2000(cvd: Cvd, colours: &[[u8; 3]]) -> f64 {
    let seen: Vec<Srgb> = colours.iter().map(|c| simulate(cvd, to_srgb(*c))).collect();
    seen.iter().enumerate().flat_map(|(i, a)| seen[i + 1..].iter().map(move |b| delta_e2000(*a, *b))).fold(f64::INFINITY, f64::min)
}
