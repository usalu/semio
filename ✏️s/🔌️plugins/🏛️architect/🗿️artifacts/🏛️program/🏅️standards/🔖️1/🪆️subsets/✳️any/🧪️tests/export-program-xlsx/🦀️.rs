//! 📕️ Architect program XLSX exporter case — production subject and calamine oracle.

use semio_repo_test_host::{Adapter, Context, Outcome};

#[cfg(feature = "sut")]
const PROGRAM: &str = "asset://🧬️schema/🧬️mutations/🌱🧱create-program-element/🧪️tests/creates-program-element-a/📸️snapshot/➡️after/🔣️.json";

fn oracle(ctx: &Context) -> Result<Outcome, String> {
    let raw = ctx.subject_raw_bytes("rust")?;
    let projection = semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::any::project_xlsx_workbook(&raw, 0)?;
    Ok(Outcome::with_raw(raw, projection))
}

#[cfg(feature = "sut")]
mod subject {
    use super::PROGRAM;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_architect::artifacts::program::io::export::serializers::artifacts::xlsx::v_ecma_376::any as export;
    use semio_s_plugin_architect::artifacts::program::standards::v1::subsets::any::schema::mutations::decode_program_snapshot_json;

    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        loop {
            match future.as_mut().poll(&mut context) {
                std::task::Poll::Ready(output) => return output,
                std::task::Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    fn value(value: &export::XlsxCellValue, shared_strings: &[String]) -> Result<Option<Json>, String> {
        match value {
            export::XlsxCellValue::Number(value) => Ok(Some(Json::Number(*value))),
            export::XlsxCellValue::SharedString(index) => shared_strings.get(*index).cloned().map(Json::String).map(Some).ok_or_else(|| format!("shared string index {index} is outside the pool")),
            export::XlsxCellValue::InlineString(value) => Ok(Some(Json::String(value.clone()))),
            export::XlsxCellValue::Boolean(value) => Ok(Some(Json::Bool(*value))),
            export::XlsxCellValue::Formula { .. } => Err("program XLSX export unexpectedly produced a formula".into()),
            export::XlsxCellValue::Empty => Ok(None),
        }
    }

    fn projection(snapshot: &export::XlsxSnapshot) -> Result<Json, String> {
        let sheets = snapshot
            .workbook
            .sheets
            .iter()
            .map(|sheet| {
                let cells = sheet
                    .cells
                    .iter()
                    .filter_map(|cell| match value(&cell.value, &snapshot.workbook.shared_strings) {
                        Ok(Some(value)) => Some(Ok(Json::Object(vec![("row".into(), Json::Number(cell.row as f64)), ("col".into(), Json::Number(cell.col as f64)), ("value".into(), value)]))),
                        Ok(None) => None,
                        Err(error) => Some(Err(error)),
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                Ok(Json::Object(vec![("name".into(), Json::String(sheet.name.clone())), ("cells".into(), Json::Array(cells))]))
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Json::Object(vec![("format".into(), Json::String("xlsx".into())), ("sharedStringCount".into(), Json::Number(0.0)), ("sheets".into(), Json::Array(sheets))]))
    }

    pub fn export_xlsx(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(PROGRAM)?).map_err(|error| format!("program fixture is not UTF-8: {error}"))?;
        let program = decode_program_snapshot_json(&text)?;
        let snapshot = block_on(export::serialize(&program)).map_err(|error| error.to_string())?;
        let raw = block_on(export::serialize_raw_bytes(&program)).map_err(|error| error.to_string())?;
        let projected = projection(&snapshot)?;
        if projected.array("sheets").len() != 70 {
            return Err(format!("expected 70 program worksheets, got {}", projected.array("sheets").len()));
        }
        Ok(Outcome::with_raw(raw, projected))
    }
}

pub fn adapter() -> Adapter {
    let built = Adapter::new("rust").oracle("export-xlsx", oracle);
    #[cfg(feature = "sut")]
    let built = built.subject("export-xlsx", subject::export_xlsx);
    built
}
