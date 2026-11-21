"""🌀️ Stages the exact empty Flate-history reservation quote through the normal object reader."""

from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"
FILTERS = Path("🔨️modules/🗜️filters")
XREF = Path("🔨️modules/🔗️xref")

PATCHES = [
    ("flate", FILTERS / "🌀️bounded-flate/🦀️.rs",
        "    /// ➡️ Advances against the same admitted source",
        """    /// 🧺️ Quotes the next unit only when it reserves the empty inflate history.
    pub(crate) fn next_reserves_history(&self) -> bool {
        self.lifecycle == Lifecycle::Decoding && self.inflater.as_ref().is_some_and(|inflater| {
            inflater.retained_allocated_bytes() == 0 && inflater.next_retained_allocation_bytes().is_some()
        })
    }

    /// ➡️ Advances against the same admitted source"""),
    ("predictor", FILTERS / "🌀️bounded-flate/🧮️predictor/🦀️.rs",
        "    /// ⚙️ Charges both stages",
        """    /// 🪟️ Quotes a fresh inflate history before any predictor output is staged.
    pub(crate) fn next_reserves_history(&self) -> bool {
        self.lifecycle == Lifecycle::Decoding && !self.input_complete && self.filtered_byte.is_none()
            && matches!(self.predicted, Predicted::Empty)
            && matches!(&self.input, PredictorInput::Flate(cursor) if cursor.next_reserves_history())
    }

    /// ⚙️ Charges both stages"""),
    ("stream-data", FILTERS / "🪣️stream-data/🦀️.rs",
        "    /// ▶️ Drives one bounded turn",
        """    /// 🪣️ Quotes the fresh history unit before copying any staged stream bytes.
    pub fn next_reserves_history(&self) -> bool {
        if self.phase != Phase::Decode || self.previous.is_some() || self.staging_start < self.staging_end || self.decoder_done { return false; }
        match &self.decoder {
            Decoder::Flate(cursor) => cursor.next_reserves_history(),
            Decoder::Predicted(cursor) => cursor.next_reserves_history(),
            _ => false,
        }
    }

    /// ▶️ Drives one bounded turn"""),
    ("decoded-stream", FILTERS / "🎚️decoded-stream/🦀️.rs",
        "    /// ▶️ Drives dictionary planning",
        """    /// 🎚️ Quotes only the fresh history unit of the currently decoding filter.
    pub fn next_reserves_history(&self) -> bool {
        !self.cancelled && !self.closing && !self.complete && !self.finishing_stage
            && self.decoder.as_ref().is_some_and(PdfStreamDataCursor::next_reserves_history)
    }

    /// ▶️ Drives dictionary planning"""),
    ("normal", XREF / "📦️normal-projection/🦀️.rs",
        "    /// ▶️ Drives one object",
        """    /// 🛂️ Quotes an empty history allocation before decoding normal stream data.
    pub fn next_reserves_history(&self) -> bool {
        matches!(self.phase, Phase::Shape(Shape::Decode)) && self.decoder.as_ref().is_some_and(PdfDecodedStreamCursor::next_reserves_history)
    }

    /// ▶️ Drives one object"""),
    ("graph", XREF / "📥️retained-graph/🦀️.rs",
        """    /// 📦️ Reports the next allocation when it reserves empty retained object slots.
    pub fn reserving_empty_objects(&self) -> bool {
        matches!(self.phase, PreparePhase::Bytes) && self.bytes.as_ref().is_some_and(|file|
            matches!(file.phase, BytePhase::Project) && file.projection.as_ref().is_some_and(|graph| matches!(graph.phase, Phase::Reserve)))
    }""",
        """    /// 🧵️ Quotes the next unit when it reserves empty retained slots or a fresh Flate history.
    pub fn reserving_empty_backing(&self) -> bool {
        matches!(self.phase, PreparePhase::Bytes) && self.bytes.as_ref().is_some_and(|file|
            matches!(file.phase, BytePhase::Project) && file.projection.as_ref().is_some_and(|graph|
                matches!(graph.phase, Phase::Reserve) || matches!(graph.phase, Phase::Normal)
                    && graph.retiring.is_none() && graph.length.is_none() && graph.length_reference.is_none() && graph.compressed_length.is_none()
                    && graph.normal.as_ref().is_some_and(|stream| stream.next_reserves_history())))
    }"""),
]


def stage(base, apply=False):
    changes = []
    for name, source, old, new in PATCHES:
        original = (base / source).read_text()
        assert original.count(old) == 1, source
        changes.append((name, source, original.replace(old, new)))
    for name, source, patched in changes:
        selected = (base / source) if apply else OUTPUT / f"🎞️{name}-history.rs"
        selected.write_text(patched)


if __name__ == "__main__":
    stage(Path(sys.argv[1]), len(sys.argv) > 2 and sys.argv[2] == "--apply")
