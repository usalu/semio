//! 📶️ Utilisation bar of the checks window: the band, the glyph-patterned text bar and the determinate progress node. See [`super`].

use super::*;

pub const UTILISATION_SCALE_MAX: f64 = 1.2;

pub const UTILISATION_BAR_CELLS: usize = 12;

/// 🎚️ Redundant coding of a check's utilisation: tone, fill pattern and status glyph, never colour alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UtilisationBand {
    Within,
    Warning,
    Over,
    NotApplicable,
    Unverified,
}

impl UtilisationBand {
    /// 🎚️ Band of a check: the kernel status decides, a utilisation above 1 always reads as over.
    pub fn of(status: CheckStatus, utilization: f64) -> Self {
        match status {
            CheckStatus::NotApplicable => Self::NotApplicable,
            CheckStatus::Fail => Self::Over,
            _ if utilization > 1.0 => Self::Over,
            CheckStatus::UnverifiedBasis => Self::Unverified,
            CheckStatus::Warning => Self::Warning,
            CheckStatus::Pass | CheckStatus::Info => Self::Within,
        }
    }

    pub fn fill(self) -> char {
        match self {
            Self::Within => '█',
            Self::Warning => '▓',
            Self::Over => '▒',
            Self::NotApplicable => '·',
            Self::Unverified => '▚',
        }
    }

    pub fn glyph(self) -> char {
        match self {
            Self::Within => '✓',
            Self::Warning => '⚠',
            Self::Over => '✗',
            Self::NotApplicable => '∅',
            Self::Unverified => '?',
        }
    }

    pub fn tone(self) -> ui::Tone {
        match self {
            Self::Within => ui::Tone::Success,
            Self::Warning | Self::Unverified => ui::Tone::Warning,
            Self::Over => ui::Tone::Danger,
            Self::NotApplicable => ui::Tone::Neutral,
        }
    }
}

/// 📶️ Number of filled cells of a `0..=120 %` bar of [`UTILISATION_BAR_CELLS`] cells.
pub fn utilisation_filled_cells(utilization: f64) -> usize {
    if !utilization.is_finite() || utilization <= 0.0 {
        return 0;
    }
    ((utilization.min(UTILISATION_SCALE_MAX) / UTILISATION_SCALE_MAX) * UTILISATION_BAR_CELLS as f64).round() as usize
}

/// 📶️ Text bar with a `┃` tick at 100 %, e.g. `█████████┃░░` for the band pattern.
pub fn utilisation_bar_text(status: CheckStatus, utilization: f64) -> String {
    let band = UtilisationBand::of(status, utilization);
    if band == UtilisationBand::NotApplicable {
        return "·".repeat(UTILISATION_BAR_CELLS);
    }
    let filled = utilisation_filled_cells(utilization);
    let tick = (UTILISATION_BAR_CELLS as f64 / UTILISATION_SCALE_MAX).round() as usize;
    let mut text = String::new();
    for cell in 0..UTILISATION_BAR_CELLS {
        if cell == tick {
            text.push('┃');
        }
        text.push(if cell < filled { band.fill() } else { '░' });
    }
    text
}

/// 📶️ Locale-aware percentage, `64 %` / `64 %` with the decimal comma for German below 10 %.
pub fn utilisation_percent_text(utilization: f64, locale: Locale) -> String {
    let text = if utilization.abs() < 0.1 { format!("{:.1}", utilization * 100.0) } else { format!("{:.0}", utilization * 100.0) };
    format!("{} %", localize_decimal(&text, locale))
}

/// 🔢️ Swaps the decimal point for the German comma.
pub fn localize_decimal(text: &str, locale: Locale) -> String {
    match locale {
        Locale::En => text.to_string(),
        Locale::De => text.replace('.', ","),
    }
}

/// 📶️ Determinate `Progress` node on the `0..=120 %` scale, toned by band, announcing the percentage and status.
pub fn utilisation_progress_node(status: CheckStatus, utilization: f64, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let band = UtilisationBand::of(status, utilization);
    let value_text = format!("{} · {}", utilisation_percent_text(utilization, locale), status_label(status, locale));
    ui::progress(utilization.clamp(0.0, UTILISATION_SCALE_MAX), norm_ui_label(value_text)?)
        .total(UTILISATION_SCALE_MAX)
        .tone(band.tone())
        .try_build()
        .map_err(|_| ui_error("ui.node.build"))
}
