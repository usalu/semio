#[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-model/🦀️component.rs"]
mod model_case;
#[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-presentation/🦀️component.rs"]
mod presentation_case;

fn main() {
    let model = model_case::adapter();
    let presentation = presentation_case::adapter();
    println!("model oracle scenarios: {}", model.registered("oracle").len());
    for id in model.registered("oracle") {
        println!("  model {id}");
    }
    println!("presentation oracle scenarios: {}", presentation.registered("oracle").len());
    for id in presentation.registered("oracle") {
        println!("  presentation {id}");
    }
}
