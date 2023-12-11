using System;
using System.Drawing;
using Grasshopper.Kernel;

namespace Semio.UI.Grasshopper;

public class ExecuteRawCommand : GH_Component
{
    public ExecuteRawCommand()
        : base("ExecuteRawCommand", "Nickname",
            "Description",
            "Category", "Subcategory")
    {
    }

    protected override Bitmap Icon =>
        //You can add image files to your project resources and access them like this:
        // return Resources.IconForThisComponent;
        null;

    public override Guid ComponentGuid => new Guid("4374F927-EECC-4999-B58E-4CE00DEF321F");

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Command", "C", "Command to execute", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Output", "O", "Output of command", GH_ParamAccess.item);
        pManager.AddTextParameter("Error", "E", "Error of command", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        string command = "";
        DA.GetData(0, ref command);

        // start process "semio.exe" and wait for exit
        var process = new System.Diagnostics.Process();
        process.StartInfo.FileName = "semio.exe";
        process.StartInfo.Arguments = command;
        process.StartInfo.UseShellExecute = false;
        process.StartInfo.RedirectStandardOutput = true;
        process.StartInfo.RedirectStandardError = true;
        process.Start();
        process.WaitForExit();

        // read output and error
        string output = process.StandardOutput.ReadToEnd();
        string error = process.StandardError.ReadToEnd();

        // set output
        DA.SetData(0, output);
        DA.SetData(1, error);

    }
}