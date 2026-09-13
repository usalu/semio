mod generated {
    include!(concat!(env!("SEMIO_REPO_ROOT"), "/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🤖️generated/🦀️.rs"));
}

#[cfg(test)]
mod tests {
    use super::generated::{Locale, Terminology};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Entry { id: String, variant: String }

    #[derive(Deserialize)]
    struct Axes { locales: Vec<Entry>, terminologies: Vec<Entry> }

    #[test]
    fn neutral_axes_and_native_projection_preserve_the_complete_two_by_two_matrix() {
        let source: Axes = serde_json::from_str(include_str!(concat!(env!("SEMIO_REPO_ROOT"), "/🧰️framework/🔨️modules/🖱️ui/🎚️axes/🔣️.json"))).unwrap();
        assert_eq!(source.locales.iter().map(|entry| entry.id.as_str()).collect::<Vec<_>>(), Locale::ALL.map(Locale::as_str));
        assert_eq!(source.terminologies.iter().map(|entry| entry.id.as_str()).collect::<Vec<_>>(), Terminology::ALL.map(Terminology::as_str));
        assert_eq!(source.locales.iter().map(|entry| entry.variant.as_str()).collect::<Vec<_>>(), ["En", "De"]);
        assert_eq!(source.terminologies.iter().map(|entry| entry.variant.as_str()).collect::<Vec<_>>(), ["Native", "Reuse"]);
        assert_eq!(Locale::COUNT * Terminology::COUNT, 4);
        for locale in Locale::ALL {
            assert_eq!(Locale::parse(locale.as_str()), Some(locale));
            assert_eq!(serde_json::from_str::<Locale>(&serde_json::to_string(&locale).unwrap()).unwrap(), locale);
        }
        for terminology in Terminology::ALL {
            assert_eq!(Terminology::parse(terminology.as_str()), Some(terminology));
            assert_eq!(serde_json::from_str::<Terminology>(&serde_json::to_string(&terminology).unwrap()).unwrap(), terminology);
        }
        assert_eq!(Locale::parse("fr"), None);
        assert_eq!(Terminology::parse("unknown"), None);
    }
}
