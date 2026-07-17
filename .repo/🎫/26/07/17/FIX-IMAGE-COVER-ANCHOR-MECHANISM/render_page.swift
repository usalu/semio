import Foundation
import PDFKit
import AppKit

let args = CommandLine.arguments
let pdfPath = args[1]
let pageIndex = Int(args[2])!
let outPath = args[3]

guard let doc = PDFDocument(url: URL(fileURLWithPath: pdfPath)) else {
    print("failed to load pdf")
    exit(1)
}
guard let page = doc.page(at: pageIndex) else {
    print("no such page")
    exit(1)
}
let bounds = page.bounds(for: .mediaBox)
let scale: CGFloat = 3.0
let size = CGSize(width: bounds.width * scale, height: bounds.height * scale)

let image = NSImage(size: size)
image.lockFocus()
if let ctx = NSGraphicsContext.current?.cgContext {
    ctx.setFillColor(NSColor.white.cgColor)
    ctx.fill(CGRect(origin: .zero, size: size))
    ctx.saveGState()
    ctx.scaleBy(x: scale, y: scale)
    page.draw(with: .mediaBox, to: ctx)
    ctx.restoreGState()
}
image.unlockFocus()

guard let tiff = image.tiffRepresentation,
      let rep = NSBitmapImageRep(data: tiff),
      let png = rep.representation(using: .png, properties: [:]) else {
    print("failed to encode png")
    exit(1)
}
try! png.write(to: URL(fileURLWithPath: outPath))
print("wrote \(outPath)")
