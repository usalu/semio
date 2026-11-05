//! 🔢️ Units: the closed unit vocabulary, SI factors, conversion, locale-aware parse and format. See [`super`].

use super::*;
use semio_ui_locale::Locale;

macro_rules! units {
    ($($variant:ident: $symbol:literal, $alias:literal, $family:ident, $factor:expr, $offset:expr, $en:literal, $de:literal;)*) => {
        /// 🔢️ Closed unit vocabulary; the symbol is the wire form. See [`UnitId::symbol`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum UnitId { $($variant),* }

        impl UnitId {
            /// 📚️ Every unit in declaration order.
            pub const ALL: &'static [UnitId] = &[$(UnitId::$variant),*];
            const ROWS: &'static [UnitRow] = &[$(UnitRow { symbol: $symbol, alias: $alias, family: UnitFamily::$family, factor: $factor, offset: $offset, en: $en, de: $de }),*];
        }
    };
}

/// 🧱️ One row of the unit table: `si = value * factor + offset`.
#[derive(Clone, Copy, Debug)]
struct UnitRow {
    symbol: &'static str,
    alias: &'static str,
    family: UnitFamily,
    factor: f64,
    offset: f64,
    en: &'static str,
    de: &'static str,
}

/// 🧬️ Physical family; units convert only inside one family.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnitFamily {
    Dimensionless, Length, LengthPerLength, Area, Volume, SecondMoment, Warping, Mass, LineDensity, AreaDensity, Density, Time,
    Rate, Temperature, Angle, Velocity, Acceleration, Force, LineLoad, VolumeLoad, Pressure, Moment, Energy, EnergyPerArea,
    Power, HeatFlux, ThermalConductivity, HeatTransfer, ThermalResistance, Effusivity, SpecificHeat, OpeningFactor, VolumeFlow, MassFlow,
    AirPermeability, FanPower, EnergyPerPerson, InverseLength, InverseArea, Illuminance, SoundLevel, Data,
}

units! {
    One: "1", "", Dimensionless, 1.0, 0.0, "dimensionless", "dimensionslos";
    Percent: "%", "", Dimensionless, 0.01, 0.0, "percent", "Prozent";
    Permille: "‰", "permille", Dimensionless, 1e-3, 0.0, "per mille", "Promille";
    Ppm: "ppm", "", Dimensionless, 1e-6, 0.0, "parts per million", "Millionstel";
    Metre: "m", "", Length, 1.0, 0.0, "metre", "Meter";
    Millimetre: "mm", "", Length, 1e-3, 0.0, "millimetre", "Millimeter";
    Centimetre: "cm", "", Length, 1e-2, 0.0, "centimetre", "Zentimeter";
    Kilometre: "km", "", Length, 1e3, 0.0, "kilometre", "Kilometer";
    Inch: "in", "", Length, 0.0254, 0.0, "inch", "Zoll";
    Foot: "ft", "", Length, 0.3048, 0.0, "foot", "Fuß";
    SquareMetrePerMetre: "m²/m", "m2/m", LengthPerLength, 1.0, 0.0, "square metre per metre", "Quadratmeter je Meter";
    SquareCentimetrePerMetre: "cm²/m", "cm2/m", LengthPerLength, 1e-4, 0.0, "square centimetre per metre", "Quadratzentimeter je Meter";
    SquareMillimetrePerMetre: "mm²/m", "mm2/m", LengthPerLength, 1e-6, 0.0, "square millimetre per metre", "Quadratmillimeter je Meter";
    SquareMillimetrePerMillimetre: "mm²/mm", "mm2/mm", LengthPerLength, 1e-3, 0.0, "square millimetre per millimetre", "Quadratmillimeter je Millimeter";
    SquareMetre: "m²", "m2", Area, 1.0, 0.0, "square metre", "Quadratmeter";
    SquareCentimetre: "cm²", "cm2", Area, 1e-4, 0.0, "square centimetre", "Quadratzentimeter";
    SquareMillimetre: "mm²", "mm2", Area, 1e-6, 0.0, "square millimetre", "Quadratmillimeter";
    CubicMetre: "m³", "m3", Volume, 1.0, 0.0, "cubic metre", "Kubikmeter";
    CubicCentimetre: "cm³", "cm3", Volume, 1e-6, 0.0, "cubic centimetre", "Kubikzentimeter";
    CubicMillimetre: "mm³", "mm3", Volume, 1e-9, 0.0, "cubic millimetre", "Kubikmillimeter";
    Litre: "L", "l", Volume, 1e-3, 0.0, "litre", "Liter";
    MetreFourth: "m⁴", "m4", SecondMoment, 1.0, 0.0, "metre to the fourth", "Meter hoch vier";
    CentimetreFourth: "cm⁴", "cm4", SecondMoment, 1e-8, 0.0, "centimetre to the fourth", "Zentimeter hoch vier";
    MillimetreFourth: "mm⁴", "mm4", SecondMoment, 1e-12, 0.0, "millimetre to the fourth", "Millimeter hoch vier";
    MetreSixth: "m⁶", "m6", Warping, 1.0, 0.0, "metre to the sixth", "Meter hoch sechs";
    CentimetreSixth: "cm⁶", "cm6", Warping, 1e-12, 0.0, "centimetre to the sixth", "Zentimeter hoch sechs";
    MillimetreSixth: "mm⁶", "mm6", Warping, 1e-18, 0.0, "millimetre to the sixth", "Millimeter hoch sechs";
    Kilogram: "kg", "", Mass, 1.0, 0.0, "kilogram", "Kilogramm";
    Gram: "g", "", Mass, 1e-3, 0.0, "gram", "Gramm";
    Tonne: "t", "", Mass, 1e3, 0.0, "tonne", "Tonne";
    KilogramPerMetre: "kg/m", "", LineDensity, 1.0, 0.0, "kilogram per metre", "Kilogramm je Meter";
    KilogramPerSquareMetre: "kg/m²", "kg/m2", AreaDensity, 1.0, 0.0, "kilogram per square metre", "Kilogramm je Quadratmeter";
    KilogramPerCubicMetre: "kg/m³", "kg/m3", Density, 1.0, 0.0, "kilogram per cubic metre", "Kilogramm je Kubikmeter";
    Second: "s", "", Time, 1.0, 0.0, "second", "Sekunde";
    Millisecond: "ms", "", Time, 1e-3, 0.0, "millisecond", "Millisekunde";
    Minute: "min", "", Time, 60.0, 0.0, "minute", "Minute";
    Hour: "h", "", Time, 3600.0, 0.0, "hour", "Stunde";
    Day: "d", "", Time, 86400.0, 0.0, "day", "Tag";
    Year: "a", "", Time, 31_557_600.0, 0.0, "year", "Jahr";
    Hertz: "Hz", "", Rate, 1.0, 0.0, "hertz", "Hertz";
    Kilohertz: "kHz", "", Rate, 1e3, 0.0, "kilohertz", "Kilohertz";
    PerHour: "1/h", "", Rate, 1.0 / 3600.0, 0.0, "per hour", "je Stunde";
    PerYear: "1/a", "", Rate, 1.0 / 31_557_600.0, 0.0, "per year", "je Jahr";
    Kelvin: "K", "", Temperature, 1.0, 0.0, "kelvin", "Kelvin";
    Celsius: "°C", "degC", Temperature, 1.0, 273.15, "degree Celsius", "Grad Celsius";
    Radian: "rad", "", Angle, 1.0, 0.0, "radian", "Radiant";
    Milliradian: "mrad", "", Angle, 1e-3, 0.0, "milliradian", "Milliradiant";
    Degree: "°", "deg", Angle, std::f64::consts::PI / 180.0, 0.0, "degree", "Grad";
    MetrePerSecond: "m/s", "", Velocity, 1.0, 0.0, "metre per second", "Meter je Sekunde";
    MillimetrePerSecond: "mm/s", "", Velocity, 1e-3, 0.0, "millimetre per second", "Millimeter je Sekunde";
    KilometrePerHour: "km/h", "", Velocity, 1.0 / 3.6, 0.0, "kilometre per hour", "Kilometer je Stunde";
    MetrePerSecondSquared: "m/s²", "m/s2", Acceleration, 1.0, 0.0, "metre per second squared", "Meter je Quadratsekunde";
    MillimetrePerSecondSquared: "mm/s²", "mm/s2", Acceleration, 1e-3, 0.0, "millimetre per second squared", "Millimeter je Quadratsekunde";
    StandardGravity: "g₀", "g0", Acceleration, 9.80665, 0.0, "standard gravity", "Normfallbeschleunigung";
    Newton: "N", "", Force, 1.0, 0.0, "newton", "Newton";
    Kilonewton: "kN", "", Force, 1e3, 0.0, "kilonewton", "Kilonewton";
    Meganewton: "MN", "", Force, 1e6, 0.0, "meganewton", "Meganewton";
    NewtonPerMetre: "N/m", "", LineLoad, 1.0, 0.0, "newton per metre", "Newton je Meter";
    KilonewtonPerMetre: "kN/m", "", LineLoad, 1e3, 0.0, "kilonewton per metre", "Kilonewton je Meter";
    NewtonPerMillimetre: "N/mm", "", LineLoad, 1e3, 0.0, "newton per millimetre", "Newton je Millimeter";
    NewtonPerCubicMetre: "N/m³", "N/m3", VolumeLoad, 1.0, 0.0, "newton per cubic metre", "Newton je Kubikmeter";
    KilonewtonPerCubicMetre: "kN/m³", "kN/m3", VolumeLoad, 1e3, 0.0, "kilonewton per cubic metre", "Kilonewton je Kubikmeter";
    Pascal: "Pa", "", Pressure, 1.0, 0.0, "pascal", "Pascal";
    Kilopascal: "kPa", "", Pressure, 1e3, 0.0, "kilopascal", "Kilopascal";
    Megapascal: "MPa", "", Pressure, 1e6, 0.0, "megapascal", "Megapascal";
    Gigapascal: "GPa", "", Pressure, 1e9, 0.0, "gigapascal", "Gigapascal";
    NewtonPerSquareMillimetre: "N/mm²", "N/mm2", Pressure, 1e6, 0.0, "newton per square millimetre", "Newton je Quadratmillimeter";
    KilonewtonPerSquareMetre: "kN/m²", "kN/m2", Pressure, 1e3, 0.0, "kilonewton per square metre", "Kilonewton je Quadratmeter";
    Bar: "bar", "", Pressure, 1e5, 0.0, "bar", "Bar";
    NewtonMetre: "N·m", "Nm", Moment, 1.0, 0.0, "newton metre", "Newtonmeter";
    KilonewtonMetre: "kN·m", "kNm", Moment, 1e3, 0.0, "kilonewton metre", "Kilonewtonmeter";
    MeganewtonMetre: "MN·m", "MNm", Moment, 1e6, 0.0, "meganewton metre", "Meganewtonmeter";
    NewtonMillimetre: "N·mm", "Nmm", Moment, 1e-3, 0.0, "newton millimetre", "Newtonmillimeter";
    NewtonMillimetrePerMillimetre: "N·mm/mm", "Nmm/mm", Force, 1.0, 0.0, "newton millimetre per millimetre", "Newtonmillimeter je Millimeter";
    KilonewtonMetrePerMetre: "kN·m/m", "kNm/m", Force, 1e3, 0.0, "kilonewton metre per metre", "Kilonewtonmeter je Meter";
    KilonewtonCentimetre: "kN·cm", "kNcm", Moment, 10.0, 0.0, "kilonewton centimetre", "Kilonewtonzentimeter";
    Joule: "J", "", Energy, 1.0, 0.0, "joule", "Joule";
    Kilojoule: "kJ", "", Energy, 1e3, 0.0, "kilojoule", "Kilojoule";
    Megajoule: "MJ", "", Energy, 1e6, 0.0, "megajoule", "Megajoule";
    WattHour: "Wh", "", Energy, 3600.0, 0.0, "watt hour", "Wattstunde";
    KilowattHour: "kWh", "", Energy, 3.6e6, 0.0, "kilowatt hour", "Kilowattstunde";
    JoulePerSquareMetre: "J/m²", "J/m2", EnergyPerArea, 1.0, 0.0, "joule per square metre", "Joule je Quadratmeter";
    MegajoulePerSquareMetre: "MJ/m²", "MJ/m2", EnergyPerArea, 1e6, 0.0, "megajoule per square metre", "Megajoule je Quadratmeter";
    KilowattHourPerSquareMetre: "kWh/m²", "kWh/m2", EnergyPerArea, 3.6e6, 0.0, "kilowatt hour per square metre", "Kilowattstunde je Quadratmeter";
    Watt: "W", "", Power, 1.0, 0.0, "watt", "Watt";
    Kilowatt: "kW", "", Power, 1e3, 0.0, "kilowatt", "Kilowatt";
    Megawatt: "MW", "", Power, 1e6, 0.0, "megawatt", "Megawatt";
    KilowattHourPerYear: "kWh/a", "", Power, 3.6e6 / 31_557_600.0, 0.0, "kilowatt hour per year", "Kilowattstunde je Jahr";
    WattPerSquareMetre: "W/m²", "W/m2", HeatFlux, 1.0, 0.0, "watt per square metre", "Watt je Quadratmeter";
    KilowattPerSquareMetre: "kW/m²", "kW/m2", HeatFlux, 1e3, 0.0, "kilowatt per square metre", "Kilowatt je Quadratmeter";
    Met: "met", "", HeatFlux, 58.15, 0.0, "metabolic equivalent", "metabolisches Äquivalent";
    WattPerMetreKelvin: "W/(m·K)", "W/(mK)", ThermalConductivity, 1.0, 0.0, "watt per metre kelvin", "Watt je Meter und Kelvin";
    WattPerSquareMetreKelvin: "W/(m²·K)", "W/(m2K)", HeatTransfer, 1.0, 0.0, "watt per square metre kelvin", "Watt je Quadratmeter und Kelvin";
    SquareMetreKelvinPerWatt: "m²·K/W", "m2K/W", ThermalResistance, 1.0, 0.0, "square metre kelvin per watt", "Quadratmeter Kelvin je Watt";
    Clo: "clo", "", ThermalResistance, 0.155, 0.0, "clo", "clo";
    JoulePerSquareMetreRootSecondKelvin: "J/(m²·s½·K)", "J/(m²s½K)", Effusivity, 1.0, 0.0, "joule per square metre root second kelvin", "Joule je Quadratmeter Wurzelsekunde Kelvin";
    RootMetre: "m½", "m^0.5", OpeningFactor, 1.0, 0.0, "root metre", "Wurzelmeter";
    CubicMetrePerSecond: "m³/s", "m3/s", VolumeFlow, 1.0, 0.0, "cubic metre per second", "Kubikmeter je Sekunde";
    CubicMetrePerHour: "m³/h", "m3/h", VolumeFlow, 1.0 / 3600.0, 0.0, "cubic metre per hour", "Kubikmeter je Stunde";
    LitrePerSecond: "L/s", "l/s", VolumeFlow, 1e-3, 0.0, "litre per second", "Liter je Sekunde";
    KilogramPerSecond: "kg/s", "", MassFlow, 1.0, 0.0, "kilogram per second", "Kilogramm je Sekunde";
    KilogramPerHour: "kg/h", "", MassFlow, 1.0 / 3600.0, 0.0, "kilogram per hour", "Kilogramm je Stunde";
    CubicMetrePerSecondSquareMetre: "m³/(s·m²)", "m3/(s*m2)", AirPermeability, 1.0, 0.0, "cubic metre per second square metre", "Kubikmeter je Sekunde und Quadratmeter";
    CubicMetrePerHourSquareMetre: "m³/(h·m²)", "m3/(h*m2)", AirPermeability, 1.0 / 3600.0, 0.0, "cubic metre per hour square metre", "Kubikmeter je Stunde und Quadratmeter";
    JoulePerKilogramKelvin: "J/(kg·K)", "J/(kgK)", SpecificHeat, 1.0, 0.0, "joule per kilogram kelvin", "Joule je Kilogramm und Kelvin";
    WattPerCubicMetrePerSecond: "W/(m³/s)", "W/(m3/s)", FanPower, 1.0, 0.0, "watt per cubic metre per second", "Watt je Kubikmeter je Sekunde";
    KilowattHourPerPersonYear: "kWh/(Person·a)", "kWh/(Person*a)", EnergyPerPerson, 3.6e6 / 31_557_600.0, 0.0, "kilowatt hour per person and year", "Kilowattstunde je Person und Jahr";
    PerMetre: "1/m", "", InverseLength, 1.0, 0.0, "per metre", "je Meter";
    PerMillimetre: "1/mm", "", InverseLength, 1e3, 0.0, "per millimetre", "je Millimeter";
    PerKilometre: "1/km", "", InverseLength, 1e-3, 0.0, "per kilometre", "je Kilometer";
    PerSquareMetre: "1/m²", "1/m2", InverseArea, 1.0, 0.0, "per square metre", "je Quadratmeter";
    PerSquareCentimetre: "1/cm²", "1/cm2", InverseArea, 1e4, 0.0, "per square centimetre", "je Quadratzentimeter";
    Lux: "lx", "", Illuminance, 1.0, 0.0, "lux", "Lux";
    Decibel: "dB", "", SoundLevel, 1.0, 0.0, "decibel", "Dezibel";
    Byte: "B", "", Data, 1.0, 0.0, "byte", "Byte";
    Kilobyte: "kB", "", Data, 1e3, 0.0, "kilobyte", "Kilobyte";
    Mebibyte: "MiB", "", Data, 1_048_576.0, 0.0, "mebibyte", "Mebibyte";
}

/// ❌️ Why a unit operation was refused.
#[derive(Clone, Debug, PartialEq)]
pub enum UnitError {
    Incompatible { from: UnitId, to: UnitId },
    UnknownUnit(String),
    InvalidNumber(String),
    Empty,
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Incompatible { from, to } => write!(f, "cannot convert {} to {}", from.symbol(), to.symbol()),
            Self::UnknownUnit(text) => write!(f, "unknown unit '{text}'"),
            Self::InvalidNumber(text) => write!(f, "invalid number '{text}'"),
            Self::Empty => f.write_str("empty input"),
        }
    }
}

/// 📏️ A value with the unit it is expressed in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Measure {
    pub value: f64,
    pub unit: UnitId,
}

/// 🎛️ Unit preset a window applies to every quantity field; `Si` keeps the stored unit, `Engineering` picks mm/kN/MPa style units.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnitSystem {
    Si,
    Engineering,
}

impl UnitId {
    fn row(self) -> &'static UnitRow {
        &Self::ROWS[self as usize]
    }

    /// 🔤️ Canonical wire and display symbol, for example `kN·m`.
    pub fn symbol(self) -> &'static str {
        self.row().symbol
    }

    /// 🌐️ Localized unit name.
    pub fn name(self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => self.row().en,
            Locale::De => self.row().de,
        }
    }

    /// 🧬️ Physical family; only units of one family convert into each other.
    pub fn family(self) -> UnitFamily {
        self.row().family
    }

    /// ⚖️ Factor `k` of `si = value * k + offset`.
    pub fn factor(self) -> f64 {
        self.row().factor
    }

    /// ↕️ Offset `o` of `si = value * k + offset`.
    pub fn offset(self) -> f64 {
        self.row().offset
    }

    /// 🔗️ Units of the same family in declaration order, this unit included.
    pub fn compatible(self) -> impl Iterator<Item = UnitId> {
        let family = self.family();
        Self::ALL.iter().copied().filter(move |unit| unit.family() == family)
    }

    /// 🔗️ The unit whose factor is exactly one in this family, if any.
    pub fn base(self) -> Option<UnitId> {
        self.compatible().find(|unit| unit.factor() == 1.0 && unit.offset() == 0.0)
    }

    /// 📊️ Matching norm quantity kind where the compliance core has one.
    pub fn quantity_kind(self) -> Option<QuantityKind> {
        use UnitFamily as F;
        Some(match self.family() {
            F::Dimensionless => QuantityKind::Dimensionless,
            F::Length => QuantityKind::Length,
            F::Area => QuantityKind::Area,
            F::Volume => QuantityKind::Volume,
            F::Mass => QuantityKind::Mass,
            F::Time => QuantityKind::Time,
            F::Temperature => QuantityKind::Temperature,
            F::Acceleration => QuantityKind::Acceleration,
            F::Velocity => QuantityKind::Velocity,
            F::Force => QuantityKind::Force,
            F::Pressure => QuantityKind::Pressure,
            F::Moment => QuantityKind::Moment,
            F::Energy | F::EnergyPerArea => QuantityKind::Energy,
            F::Power => QuantityKind::Power,
            F::ThermalConductivity => QuantityKind::ThermalConductivity,
            F::HeatTransfer => QuantityKind::HeatTransferCoefficient,
            F::ThermalResistance => QuantityKind::ThermalResistance,
            F::AirPermeability => QuantityKind::AirPermeability,
            _ => return None,
        })
    }

    /// ➡️ Converts `value` of this unit to the SI base of its family.
    pub fn to_si(self, value: f64) -> f64 {
        value * self.factor() + self.offset()
    }

    /// ⬅️ Converts an SI-base `value` to this unit.
    pub fn from_si(self, value: f64) -> f64 {
        (value - self.offset()) / self.factor()
    }

    /// 🔁️ Converts `value` from this unit to `to`, refusing across families.
    pub fn convert(self, value: f64, to: UnitId) -> Result<f64, UnitError> {
        if self.family() != to.family() {
            return Err(UnitError::Incompatible { from: self, to });
        }
        Ok(if self == to { value } else { to.from_si(self.to_si(value)) })
    }

    /// 🔎️ Resolves a symbol, ASCII alias or localized name (case-insensitive for names).
    pub fn lookup(text: &str) -> Option<UnitId> {
        let text = text.trim().replace('*', "·");
        let direct = |t: &str| Self::ROWS.iter().position(|r| r.symbol == t || (!r.alias.is_empty() && r.alias == t));
        let found = direct(&text).or_else(|| direct(&text.replace('·', "*"))).or_else(|| {
            let lower = text.to_lowercase();
            Self::ROWS.iter().position(|r| r.en.to_lowercase() == lower || r.de.to_lowercase() == lower)
        });
        found.map(|index| Self::ALL[index])
    }

    /// 🏗️ Engineering display unit for the family under the preset, falling back to this unit.
    pub fn in_system(self, system: UnitSystem) -> UnitId {
        use UnitId as U;
        match (system, self.family()) {
            (UnitSystem::Si, _) => self,
            (_, UnitFamily::Length) => U::Millimetre,
            (_, UnitFamily::Area) => U::SquareMillimetre,
            (_, UnitFamily::Force) => U::Kilonewton,
            (_, UnitFamily::Pressure) => U::Megapascal,
            (_, UnitFamily::Moment) => U::KilonewtonMetre,
            (_, UnitFamily::LineLoad) => U::KilonewtonPerMetre,
            (_, UnitFamily::SecondMoment) => U::MillimetreFourth,
            (_, UnitFamily::Angle) => U::Degree,
            _ => self,
        }
    }
}

impl Measure {
    /// ➡️ Value in the SI base of the unit family.
    pub fn si(self) -> f64 {
        self.unit.to_si(self.value)
    }

    /// 🔁️ Value expressed in `unit`.
    pub fn to(self, unit: UnitId) -> Result<f64, UnitError> {
        self.unit.convert(self.value, unit)
    }

    /// 📖️ Parses `12.5 kN`, `12,5kN`, `1 250 mm` or a bare number (which takes `fallback`); both decimal marks are accepted.
    pub fn parse(text: &str, fallback: UnitId) -> Result<Self, UnitError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(UnitError::Empty);
        }
        let numeric = |index: usize, c: char| {
            c.is_ascii_digit()
                || matches!(c, '.' | ',' | '+' | '-' | ' ' | '\u{2009}' | '\u{202f}' | '\u{a0}')
                || (matches!(c, 'e' | 'E') && index > 0 && text[index + 1..].starts_with(|n: char| n.is_ascii_digit() || n == '-' || n == '+'))
        };
        let split = text.char_indices().find(|&(index, c)| !numeric(index, c)).map_or(text.len(), |(index, _)| index);
        let (number, rest) = text.split_at(split);
        let value = parse_number(number).ok_or_else(|| UnitError::InvalidNumber(number.trim().to_owned()))?;
        let rest = rest.trim();
        let unit = if rest.is_empty() { fallback } else { UnitId::lookup(rest).ok_or_else(|| UnitError::UnknownUnit(rest.to_owned()))? };
        Ok(Self { value, unit })
    }

    /// 🖨️ Formats with fixed `precision` decimals, locale decimal mark, thin-space grouping from five integer digits and a no-break space before the symbol.
    pub fn format(self, precision: u8, locale: Locale) -> String {
        format!("{}\u{a0}{}", format_number(self.value, precision, locale), self.unit.symbol())
    }
}

fn parse_number(text: &str) -> Option<f64> {
    let cleaned: String = text.chars().filter(|c| !matches!(c, ' ' | '\u{2009}' | '\u{202f}' | '\u{a0}')).collect();
    let decimal = match (cleaned.rfind('.'), cleaned.rfind(',')) {
        (Some(dot), Some(comma)) => Some(dot.max(comma)),
        (Some(dot), None) => (cleaned.matches('.').count() == 1).then_some(dot),
        (None, Some(comma)) => (cleaned.matches(',').count() == 1).then_some(comma),
        (None, None) => None,
    };
    let normalized: String = cleaned.char_indices().filter_map(|(i, c)| match c {
        '.' | ',' => (Some(i) == decimal).then_some('.'),
        c => Some(c),
    }).collect();
    normalized.parse::<f64>().ok().filter(|v| v.is_finite())
}

/// 🔢️ Locale-aware number text: fixed decimals, comma decimal mark for German, thin-space grouping from five integer digits.
pub fn format_number(value: f64, precision: u8, locale: Locale) -> String {
    let fixed = format!("{:.*}", precision as usize, value);
    let (sign, digits) = fixed.strip_prefix('-').map_or(("", fixed.as_str()), |d| ("-", d));
    let (int, frac) = digits.split_once('.').map_or((digits, ""), |(i, f)| (i, f));
    let grouped = if int.len() >= 5 {
        let head = int.len() % 3;
        let mut out = String::new();
        for (i, c) in int.chars().enumerate() {
            if i > 0 && (i + 3 - head) % 3 == 0 {
                out.push('\u{2009}');
            }
            out.push(c);
        }
        out
    } else {
        int.to_owned()
    };
    let mark = match locale {
        Locale::En => '.',
        Locale::De => ',',
    };
    let negative_zero = sign == "-" && !digits.chars().any(|c| c.is_ascii_digit() && c != '0');
    format!("{}{}{}{}", if negative_zero { "" } else { sign }, grouped, if frac.is_empty() { String::new() } else { mark.to_string() }, frac)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
