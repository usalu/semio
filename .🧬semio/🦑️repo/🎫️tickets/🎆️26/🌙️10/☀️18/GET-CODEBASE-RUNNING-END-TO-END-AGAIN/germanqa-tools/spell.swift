import AppKit
let checker = NSSpellChecker.shared
checker.setLanguage("de")
while let line = readLine() {
    let word = line.trimmingCharacters(in: .whitespaces)
    if word.isEmpty { continue }
    let range = checker.checkSpelling(of: word, startingAt: 0, language: "de", wrap: false, inSpellDocumentWithTag: 0, wordCount: nil)
    if range.location != NSNotFound { print(word) }
}
