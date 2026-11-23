import Foundation
import PDFKit
import AppKit
let a = CommandLine.arguments
let doc = PDFDocument(url: URL(fileURLWithPath: a[1]))!
let page = Int(a[2])!
let scale = CGFloat(Double(a.count > 4 ? a[4] : "2")!)
let p = doc.page(at: page-1)!
let r = p.bounds(for: .mediaBox)
let w = Int(r.width*scale), h = Int(r.height*scale)
let cs = CGColorSpaceCreateDeviceRGB()
let ctx = CGContext(data: nil, width: w, height: h, bitsPerComponent: 8, bytesPerRow: 0, space: cs, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)!
ctx.setFillColor(CGColor(red:1,green:1,blue:1,alpha:1)); ctx.fill(CGRect(x:0,y:0,width:w,height:h))
ctx.scaleBy(x: scale, y: scale)
p.draw(with: .mediaBox, to: ctx)
let img = ctx.makeImage()!
let rep = NSBitmapImageRep(cgImage: img)
try! rep.representation(using: .png, properties: [:])!.write(to: URL(fileURLWithPath: a[3]))
print(doc.pageCount)
